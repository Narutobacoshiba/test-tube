package testenv

import (
	"encoding/json"
	"fmt"
	"time"

	// helpers
	"cosmossdk.io/math"
	"github.com/cosmos/cosmos-sdk/baseapp"
	codectypes "github.com/cosmos/cosmos-sdk/codec/types"
	cryptocodec "github.com/cosmos/cosmos-sdk/crypto/codec"

	// tendermint
	dbm "github.com/cometbft/cometbft-db"
	abci "github.com/cometbft/cometbft/abci/types"
	"github.com/cometbft/cometbft/libs/log"
	tmproto "github.com/cometbft/cometbft/proto/tendermint/types"
	cmtypes "github.com/cometbft/cometbft/types"
	cryptotypes "github.com/cosmos/cosmos-sdk/crypto/types"
	authtypes "github.com/cosmos/cosmos-sdk/x/auth/types"
	banktypes "github.com/cosmos/cosmos-sdk/x/bank/types"

	// cosmos-sdk
	"github.com/cosmos/cosmos-sdk/crypto/keys/secp256k1"
	"github.com/cosmos/cosmos-sdk/server"
	sdk "github.com/cosmos/cosmos-sdk/types"
	bankkeeper "github.com/cosmos/cosmos-sdk/x/bank/keeper"
	minttypes "github.com/cosmos/cosmos-sdk/x/mint/types"
	slashingtypes "github.com/cosmos/cosmos-sdk/x/slashing/types"
	stakingtypes "github.com/cosmos/cosmos-sdk/x/staking/types"

	// wasmd

	wasmtypes "github.com/CosmWasm/wasmd/x/wasm/types"

	// aura
	"github.com/aura-nw/aura/app"
	smartaccounttypes "github.com/aura-nw/aura/x/smartaccount/types/v1beta1"
)

type TestEnv struct {
	App                *app.App
	Ctx                sdk.Context
	ParamTypesRegistry ParamTypeRegistry
}

const ChainID = "aura-testnet"
const BondDenom = "uaura"

var DefaultConsensusParams = &tmproto.ConsensusParams{
	Block: &tmproto.BlockParams{
		MaxBytes: 22020096,
		MaxGas:   -1,
	},
	Evidence: &tmproto.EvidenceParams{
		MaxAgeNumBlocks: 302400,
		MaxAgeDuration:  504 * time.Hour, // 3 weeks is the max duration
		MaxBytes:        10000,
	},
	Validator: &tmproto.ValidatorParams{
		PubKeyTypes: []string{
			cmtypes.ABCIPubKeyTypeEd25519,
		},
	},
}

// DebugAppOptions is a stub implementing AppOptions
type DebugAppOptions struct{}

// Get implements AppOptions
func (ao DebugAppOptions) Get(o string) interface{} {
	if o == server.FlagTrace {
		return true
	}
	return nil
}

func FundAccount(bankKeeper bankkeeper.Keeper, ctx sdk.Context, addr sdk.AccAddress, amounts sdk.Coins) error {
	if err := bankKeeper.MintCoins(ctx, minttypes.ModuleName, amounts); err != nil {
		return err
	}

	return bankKeeper.SendCoinsFromModuleToAccount(ctx, minttypes.ModuleName, addr, amounts)
}

func SetupAuraApp() *app.App {
	db := dbm.NewMemDB()
	appObj := app.New(
		log.NewNopLogger(),
		db,
		nil,
		true,
		map[int64]bool{},
		app.DefaultNodeHome,
		0,
		app.MakeEncodingConfig(),
		DebugAppOptions{},
		baseapp.SetChainID(ChainID),
	)

	privVal := NewPV()
	pubKey, err := privVal.GetPubKey()
	requireNoErr(err)

	// create validator set with single validator
	validator := cmtypes.NewValidator(pubKey, 1)
	valSet := cmtypes.NewValidatorSet([]*cmtypes.Validator{validator})

	// generate genesis account
	senderPrivKey := secp256k1.GenPrivKey()
	acc := authtypes.NewBaseAccount(senderPrivKey.PubKey().Address().Bytes(), senderPrivKey.PubKey(), 0, 0)
	balance := banktypes.Balance{
		Address: acc.GetAddress().String(),
		Coins:   sdk.NewCoins(sdk.NewCoin(BondDenom, sdk.NewInt(100000000000000))),
	}

	encConfig := app.MakeEncodingConfig()
	genesisState := genesisStateWithValSet(appObj, app.NewDefaultGenesisState(encConfig.Marshaler), valSet, []authtypes.GenesisAccount{acc}, balance)

	wasmGen := wasmtypes.GenesisState{
		Params: wasmtypes.Params{
			// Allow store code without gov
			CodeUploadAccess:             wasmtypes.AllowEverybody,
			InstantiateDefaultPermission: wasmtypes.AccessTypeEverybody,
		},
	}

	genesisState[wasmtypes.ModuleName] = encConfig.Marshaler.MustMarshalJSON(&wasmGen)

	stateBytes, err := json.MarshalIndent(genesisState, "", " ")
	requireNoErr(err)

	appObj.InitChain(
		abci.RequestInitChain{
			Validators:      []abci.ValidatorUpdate{},
			ConsensusParams: DefaultConsensusParams,
			AppStateBytes:   stateBytes,
			ChainId:         ChainID,
		},
	)

	return appObj
}

func genesisStateWithValSet(
	app *app.App, genesisState app.GenesisState,
	valSet *cmtypes.ValidatorSet, genAccs []authtypes.GenesisAccount,
	balances ...banktypes.Balance,
) app.GenesisState {
	codec := app.AppCodec()

	// set genesis accounts
	authGenesis := authtypes.NewGenesisState(authtypes.DefaultParams(), genAccs)
	genesisState[authtypes.ModuleName] = codec.MustMarshalJSON(authGenesis)

	validators := make([]stakingtypes.Validator, 0, len(valSet.Validators))
	delegations := make([]stakingtypes.Delegation, 0, len(valSet.Validators))

	bondAmt := sdk.DefaultPowerReduction

	for _, val := range valSet.Validators {
		pk, err := cryptocodec.FromTmPubKeyInterface(val.PubKey)
		requireNoErr(err)

		pkAny, err := codectypes.NewAnyWithValue(pk)
		requireNoErr(err)

		validator := stakingtypes.Validator{
			OperatorAddress:   sdk.ValAddress(val.Address).String(),
			ConsensusPubkey:   pkAny,
			Jailed:            false,
			Status:            stakingtypes.Bonded,
			Tokens:            bondAmt,
			DelegatorShares:   math.LegacyOneDec(),
			Description:       stakingtypes.Description{},
			UnbondingHeight:   int64(0),
			UnbondingTime:     time.Unix(0, 0).UTC(),
			Commission:        stakingtypes.NewCommission(math.LegacyZeroDec(), math.LegacyZeroDec(), math.LegacyZeroDec()),
			MinSelfDelegation: math.ZeroInt(),
		}
		validators = append(validators, validator)
		delegations = append(delegations, stakingtypes.NewDelegation(genAccs[0].GetAddress(), val.Address.Bytes(), math.LegacyOneDec()))

	}

	defaultStParams := stakingtypes.DefaultParams()
	stParams := stakingtypes.NewParams(
		defaultStParams.UnbondingTime,
		defaultStParams.MaxValidators,
		defaultStParams.MaxEntries,
		defaultStParams.HistoricalEntries,
		BondDenom,
		defaultStParams.MinCommissionRate, // 5%
	)

	// set validators and delegations
	stakingGenesis := stakingtypes.NewGenesisState(stParams, validators, delegations)
	genesisState[stakingtypes.ModuleName] = codec.MustMarshalJSON(stakingGenesis)

	// add bonded amount to bonded pool module account
	balances = append(balances, banktypes.Balance{
		Address: authtypes.NewModuleAddress(stakingtypes.BondedPoolName).String(),
		Coins:   sdk.Coins{sdk.NewCoin(BondDenom, bondAmt.MulRaw(int64(len(valSet.Validators))))},
	})

	totalSupply := sdk.NewCoins()
	for _, b := range balances {
		// add genesis acc tokens to total supply
		totalSupply = totalSupply.Add(b.Coins...)
	}

	// update total supply
	bankGenesis := banktypes.NewGenesisState(banktypes.DefaultGenesisState().Params, balances, totalSupply, []banktypes.Metadata{}, []banktypes.SendEnabled{})
	genesisState[banktypes.ModuleName] = codec.MustMarshalJSON(bankGenesis)
	// println("genesisStateWithValSet bankState:", string(genesisState[banktypes.ModuleName]))

	return genesisState
}

func (env *TestEnv) BeginNewBlock(beginBlock bool) {

	var valAddr []byte

	validators := env.App.StakingKeeper.GetAllValidators(env.Ctx)
	requierTrue("validator", len(validators) >= 1)

	valAddrFancy, err := validators[0].GetConsAddr()
	requireNoErr(err)
	valAddr = valAddrFancy.Bytes()

	newBlockTime := env.Ctx.BlockTime().Add(5 * time.Second)
	if beginBlock {
		newBlockTime = time.UnixMilli(0)
	}

	header := tmproto.Header{ChainID: ChainID, Height: env.Ctx.BlockHeight() + 1, Time: newBlockTime}
	newCtx := env.Ctx.WithBlockTime(newBlockTime).WithBlockHeight(env.Ctx.BlockHeight() + 1)
	env.Ctx = newCtx
	lastCommitInfo := abci.CommitInfo{
		Votes: []abci.VoteInfo{{
			Validator:       abci.Validator{Address: valAddr, Power: 1000},
			SignedLastBlock: true,
		}},
	}
	reqBeginBlock := abci.RequestBeginBlock{Header: header, LastCommitInfo: lastCommitInfo}

	env.App.BeginBlock(reqBeginBlock)
	env.Ctx = env.App.NewContext(false, reqBeginBlock.Header)
}

func (env *TestEnv) SetupValidator(bondStatus stakingtypes.BondStatus) {
	validators := env.App.StakingKeeper.GetAllValidators(env.Ctx)
	requierTrue("validator", len(validators) >= 1)

	val := validators[0]
	valPub, err := PubKeyDecode(val.ConsensusPubkey)
	requireNoErr(err)
	bondDenom := env.App.StakingKeeper.GetParams(env.Ctx).BondDenom
	selfBond := sdk.NewCoins(sdk.Coin{Amount: sdk.NewInt(100), Denom: bondDenom})

	err = FundAccount(env.App.BankKeeper, env.Ctx, sdk.AccAddress(valPub.Address()), selfBond)
	requireNoErr(err)

	stakingCoin := sdk.NewCoin(bondDenom, selfBond[0].Amount)

	env.App.BankKeeper.SendCoinsFromModuleToModule(env.Ctx, stakingtypes.NotBondedPoolName, stakingtypes.BondedPoolName, sdk.NewCoins(stakingCoin))

	val = val.UpdateStatus(bondStatus)
	env.App.StakingKeeper.SetValidator(env.Ctx, val)

	consAddr, err := val.GetConsAddr()
	requireNoErr(err)

	signingInfo := slashingtypes.NewValidatorSigningInfo(
		consAddr,
		env.Ctx.BlockHeight(),
		0,
		time.Unix(0, 0),
		false,
		0,
	)
	env.App.SlashingKeeper.SetValidatorSigningInfo(env.Ctx, consAddr, signingInfo)
}

func (env *TestEnv) SetupParamTypes() {
	pReg := env.ParamTypesRegistry
	pReg.RegisterParamSet(&smartaccounttypes.Params{})
}

func PubKeyDecode(pubKey *codectypes.Any) (cryptotypes.PubKey, error) {
	if pubKey == nil {
		return nil, fmt.Errorf("Nil PubKey")
	}

	pkAny := pubKey.GetCachedValue()
	pk, ok := pkAny.(cryptotypes.PubKey)
	if ok {
		return pk, nil
	} else {
		return nil, fmt.Errorf("expecting PubKey, got: %T", pkAny)
	}
}

func requireNoErr(err error) {
	if err != nil {
		panic(err)
	}
}

func requireNoNil(name string, nilable any) {
	if nilable == nil {
		panic(fmt.Sprintf("%s must not be nil", name))
	}
}

func requierTrue(name string, b bool) {
	if !b {
		panic(fmt.Sprintf("%s must be true", name))
	}
}
