pub use gateway_getter_facet::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types,
)]
pub mod gateway_getter_facet {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"appliedTopDownNonce\",\"outputs\":[{\"internalType\":\"uint64\",\"name\":\"\",\"type\":\"uint64\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"bottomUpCheckPeriod\",\"outputs\":[{\"internalType\":\"uint64\",\"name\":\"\",\"type\":\"uint64\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint64\",\"name\":\"e\",\"type\":\"uint64\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"bottomUpCheckpoints\",\"outputs\":[{\"internalType\":\"struct BottomUpCheckpoint\",\"name\":\"\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"struct SubnetID\",\"name\":\"source\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint64\",\"name\":\"root\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"address[]\",\"name\":\"route\",\"type\":\"address[]\",\"components\":[]}]},{\"internalType\":\"uint64\",\"name\":\"epoch\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"fee\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"struct CrossMsg[]\",\"name\":\"crossMsgs\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"struct StorableMsg\",\"name\":\"message\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"struct IPCAddress\",\"name\":\"from\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"struct SubnetID\",\"name\":\"subnetId\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint64\",\"name\":\"root\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"address[]\",\"name\":\"route\",\"type\":\"address[]\",\"components\":[]}]},{\"internalType\":\"struct FvmAddress\",\"name\":\"rawAddress\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint8\",\"name\":\"addrType\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"payload\",\"type\":\"bytes\",\"components\":[]}]}]},{\"internalType\":\"struct IPCAddress\",\"name\":\"to\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"struct SubnetID\",\"name\":\"subnetId\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint64\",\"name\":\"root\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"address[]\",\"name\":\"route\",\"type\":\"address[]\",\"components\":[]}]},{\"internalType\":\"struct FvmAddress\",\"name\":\"rawAddress\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint8\",\"name\":\"addrType\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"payload\",\"type\":\"bytes\",\"components\":[]}]}]},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"nonce\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"bytes4\",\"name\":\"method\",\"type\":\"bytes4\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"params\",\"type\":\"bytes\",\"components\":[]}]},{\"internalType\":\"bool\",\"name\":\"wrapped\",\"type\":\"bool\",\"components\":[]}]},{\"internalType\":\"struct ChildCheck[]\",\"name\":\"children\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"struct SubnetID\",\"name\":\"source\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint64\",\"name\":\"root\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"address[]\",\"name\":\"route\",\"type\":\"address[]\",\"components\":[]}]},{\"internalType\":\"bytes32[]\",\"name\":\"checks\",\"type\":\"bytes32[]\",\"components\":[]}]},{\"internalType\":\"bytes32\",\"name\":\"prevHash\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"proof\",\"type\":\"bytes\",\"components\":[]}]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"bottomUpNonce\",\"outputs\":[{\"internalType\":\"uint64\",\"name\":\"\",\"type\":\"uint64\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"crossMsgFee\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"executableQueue\",\"outputs\":[{\"internalType\":\"uint64\",\"name\":\"\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"\",\"type\":\"uint64\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct SubnetID\",\"name\":\"subnetId\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint64\",\"name\":\"root\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"address[]\",\"name\":\"route\",\"type\":\"address[]\",\"components\":[]}]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getAppliedTopDownNonce\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"\",\"type\":\"uint64\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getGenesisEpoch\",\"outputs\":[{\"internalType\":\"uint64\",\"name\":\"\",\"type\":\"uint64\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getNetworkName\",\"outputs\":[{\"internalType\":\"struct SubnetID\",\"name\":\"\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint64\",\"name\":\"root\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"address[]\",\"name\":\"route\",\"type\":\"address[]\",\"components\":[]}]}]},{\"inputs\":[{\"internalType\":\"struct SubnetID\",\"name\":\"subnetId\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint64\",\"name\":\"root\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"address[]\",\"name\":\"route\",\"type\":\"address[]\",\"components\":[]}]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getSubnet\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"struct Subnet\",\"name\":\"\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"stake\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"genesisEpoch\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"circSupply\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"topDownNonce\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"appliedBottomUpNonce\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"enum Status\",\"name\":\"status\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"struct SubnetID\",\"name\":\"id\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint64\",\"name\":\"root\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"address[]\",\"name\":\"route\",\"type\":\"address[]\",\"components\":[]}]},{\"internalType\":\"struct BottomUpCheckpoint\",\"name\":\"prevCheckpoint\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"struct SubnetID\",\"name\":\"source\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint64\",\"name\":\"root\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"address[]\",\"name\":\"route\",\"type\":\"address[]\",\"components\":[]}]},{\"internalType\":\"uint64\",\"name\":\"epoch\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"fee\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"struct CrossMsg[]\",\"name\":\"crossMsgs\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"struct StorableMsg\",\"name\":\"message\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"struct IPCAddress\",\"name\":\"from\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"struct SubnetID\",\"name\":\"subnetId\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint64\",\"name\":\"root\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"address[]\",\"name\":\"route\",\"type\":\"address[]\",\"components\":[]}]},{\"internalType\":\"struct FvmAddress\",\"name\":\"rawAddress\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint8\",\"name\":\"addrType\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"payload\",\"type\":\"bytes\",\"components\":[]}]}]},{\"internalType\":\"struct IPCAddress\",\"name\":\"to\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"struct SubnetID\",\"name\":\"subnetId\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint64\",\"name\":\"root\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"address[]\",\"name\":\"route\",\"type\":\"address[]\",\"components\":[]}]},{\"internalType\":\"struct FvmAddress\",\"name\":\"rawAddress\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint8\",\"name\":\"addrType\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"payload\",\"type\":\"bytes\",\"components\":[]}]}]},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"nonce\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"bytes4\",\"name\":\"method\",\"type\":\"bytes4\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"params\",\"type\":\"bytes\",\"components\":[]}]},{\"internalType\":\"bool\",\"name\":\"wrapped\",\"type\":\"bool\",\"components\":[]}]},{\"internalType\":\"struct ChildCheck[]\",\"name\":\"children\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"struct SubnetID\",\"name\":\"source\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint64\",\"name\":\"root\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"address[]\",\"name\":\"route\",\"type\":\"address[]\",\"components\":[]}]},{\"internalType\":\"bytes32[]\",\"name\":\"checks\",\"type\":\"bytes32[]\",\"components\":[]}]},{\"internalType\":\"bytes32\",\"name\":\"prevHash\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"proof\",\"type\":\"bytes\",\"components\":[]}]},{\"internalType\":\"struct CrossMsg[]\",\"name\":\"topDownMsgs\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"struct StorableMsg\",\"name\":\"message\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"struct IPCAddress\",\"name\":\"from\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"struct SubnetID\",\"name\":\"subnetId\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint64\",\"name\":\"root\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"address[]\",\"name\":\"route\",\"type\":\"address[]\",\"components\":[]}]},{\"internalType\":\"struct FvmAddress\",\"name\":\"rawAddress\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint8\",\"name\":\"addrType\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"payload\",\"type\":\"bytes\",\"components\":[]}]}]},{\"internalType\":\"struct IPCAddress\",\"name\":\"to\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"struct SubnetID\",\"name\":\"subnetId\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint64\",\"name\":\"root\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"address[]\",\"name\":\"route\",\"type\":\"address[]\",\"components\":[]}]},{\"internalType\":\"struct FvmAddress\",\"name\":\"rawAddress\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint8\",\"name\":\"addrType\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"payload\",\"type\":\"bytes\",\"components\":[]}]}]},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"nonce\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"bytes4\",\"name\":\"method\",\"type\":\"bytes4\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"params\",\"type\":\"bytes\",\"components\":[]}]},{\"internalType\":\"bool\",\"name\":\"wrapped\",\"type\":\"bool\",\"components\":[]}]}]}]},{\"inputs\":[{\"internalType\":\"struct SubnetID\",\"name\":\"subnetId\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint64\",\"name\":\"root\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"address[]\",\"name\":\"route\",\"type\":\"address[]\",\"components\":[]}]},{\"internalType\":\"uint256\",\"name\":\"index\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getSubnetTopDownMsg\",\"outputs\":[{\"internalType\":\"struct CrossMsg\",\"name\":\"\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"struct StorableMsg\",\"name\":\"message\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"struct IPCAddress\",\"name\":\"from\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"struct SubnetID\",\"name\":\"subnetId\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint64\",\"name\":\"root\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"address[]\",\"name\":\"route\",\"type\":\"address[]\",\"components\":[]}]},{\"internalType\":\"struct FvmAddress\",\"name\":\"rawAddress\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint8\",\"name\":\"addrType\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"payload\",\"type\":\"bytes\",\"components\":[]}]}]},{\"internalType\":\"struct IPCAddress\",\"name\":\"to\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"struct SubnetID\",\"name\":\"subnetId\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint64\",\"name\":\"root\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"address[]\",\"name\":\"route\",\"type\":\"address[]\",\"components\":[]}]},{\"internalType\":\"struct FvmAddress\",\"name\":\"rawAddress\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint8\",\"name\":\"addrType\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"payload\",\"type\":\"bytes\",\"components\":[]}]}]},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"nonce\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"bytes4\",\"name\":\"method\",\"type\":\"bytes4\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"params\",\"type\":\"bytes\",\"components\":[]}]},{\"internalType\":\"bool\",\"name\":\"wrapped\",\"type\":\"bool\",\"components\":[]}]}]},{\"inputs\":[{\"internalType\":\"struct SubnetID\",\"name\":\"subnetId\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint64\",\"name\":\"root\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"address[]\",\"name\":\"route\",\"type\":\"address[]\",\"components\":[]}]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getSubnetTopDownMsgsLength\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct SubnetID\",\"name\":\"subnetId\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint64\",\"name\":\"root\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"address[]\",\"name\":\"route\",\"type\":\"address[]\",\"components\":[]}]},{\"internalType\":\"uint64\",\"name\":\"fromNonce\",\"type\":\"uint64\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getTopDownMsgs\",\"outputs\":[{\"internalType\":\"struct CrossMsg[]\",\"name\":\"\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"struct StorableMsg\",\"name\":\"message\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"struct IPCAddress\",\"name\":\"from\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"struct SubnetID\",\"name\":\"subnetId\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint64\",\"name\":\"root\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"address[]\",\"name\":\"route\",\"type\":\"address[]\",\"components\":[]}]},{\"internalType\":\"struct FvmAddress\",\"name\":\"rawAddress\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint8\",\"name\":\"addrType\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"payload\",\"type\":\"bytes\",\"components\":[]}]}]},{\"internalType\":\"struct IPCAddress\",\"name\":\"to\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"struct SubnetID\",\"name\":\"subnetId\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint64\",\"name\":\"root\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"address[]\",\"name\":\"route\",\"type\":\"address[]\",\"components\":[]}]},{\"internalType\":\"struct FvmAddress\",\"name\":\"rawAddress\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint8\",\"name\":\"addrType\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"payload\",\"type\":\"bytes\",\"components\":[]}]}]},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"nonce\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"bytes4\",\"name\":\"method\",\"type\":\"bytes4\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"params\",\"type\":\"bytes\",\"components\":[]}]},{\"internalType\":\"bool\",\"name\":\"wrapped\",\"type\":\"bool\",\"components\":[]}]}]},{\"inputs\":[{\"internalType\":\"uint64\",\"name\":\"epoch\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"submitter\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"hasValidatorVotedForSubmission\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"initialized\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"lastVotingExecutedEpoch\",\"outputs\":[{\"internalType\":\"uint64\",\"name\":\"\",\"type\":\"uint64\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"listSubnets\",\"outputs\":[{\"internalType\":\"struct Subnet[]\",\"name\":\"\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"stake\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"genesisEpoch\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"circSupply\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"topDownNonce\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"appliedBottomUpNonce\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"enum Status\",\"name\":\"status\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"struct SubnetID\",\"name\":\"id\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint64\",\"name\":\"root\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"address[]\",\"name\":\"route\",\"type\":\"address[]\",\"components\":[]}]},{\"internalType\":\"struct BottomUpCheckpoint\",\"name\":\"prevCheckpoint\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"struct SubnetID\",\"name\":\"source\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint64\",\"name\":\"root\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"address[]\",\"name\":\"route\",\"type\":\"address[]\",\"components\":[]}]},{\"internalType\":\"uint64\",\"name\":\"epoch\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"fee\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"struct CrossMsg[]\",\"name\":\"crossMsgs\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"struct StorableMsg\",\"name\":\"message\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"struct IPCAddress\",\"name\":\"from\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"struct SubnetID\",\"name\":\"subnetId\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint64\",\"name\":\"root\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"address[]\",\"name\":\"route\",\"type\":\"address[]\",\"components\":[]}]},{\"internalType\":\"struct FvmAddress\",\"name\":\"rawAddress\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint8\",\"name\":\"addrType\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"payload\",\"type\":\"bytes\",\"components\":[]}]}]},{\"internalType\":\"struct IPCAddress\",\"name\":\"to\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"struct SubnetID\",\"name\":\"subnetId\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint64\",\"name\":\"root\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"address[]\",\"name\":\"route\",\"type\":\"address[]\",\"components\":[]}]},{\"internalType\":\"struct FvmAddress\",\"name\":\"rawAddress\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint8\",\"name\":\"addrType\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"payload\",\"type\":\"bytes\",\"components\":[]}]}]},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"nonce\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"bytes4\",\"name\":\"method\",\"type\":\"bytes4\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"params\",\"type\":\"bytes\",\"components\":[]}]},{\"internalType\":\"bool\",\"name\":\"wrapped\",\"type\":\"bool\",\"components\":[]}]},{\"internalType\":\"struct ChildCheck[]\",\"name\":\"children\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"struct SubnetID\",\"name\":\"source\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint64\",\"name\":\"root\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"address[]\",\"name\":\"route\",\"type\":\"address[]\",\"components\":[]}]},{\"internalType\":\"bytes32[]\",\"name\":\"checks\",\"type\":\"bytes32[]\",\"components\":[]}]},{\"internalType\":\"bytes32\",\"name\":\"prevHash\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"proof\",\"type\":\"bytes\",\"components\":[]}]},{\"internalType\":\"struct CrossMsg[]\",\"name\":\"topDownMsgs\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"struct StorableMsg\",\"name\":\"message\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"struct IPCAddress\",\"name\":\"from\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"struct SubnetID\",\"name\":\"subnetId\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint64\",\"name\":\"root\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"address[]\",\"name\":\"route\",\"type\":\"address[]\",\"components\":[]}]},{\"internalType\":\"struct FvmAddress\",\"name\":\"rawAddress\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint8\",\"name\":\"addrType\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"payload\",\"type\":\"bytes\",\"components\":[]}]}]},{\"internalType\":\"struct IPCAddress\",\"name\":\"to\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"struct SubnetID\",\"name\":\"subnetId\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint64\",\"name\":\"root\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"address[]\",\"name\":\"route\",\"type\":\"address[]\",\"components\":[]}]},{\"internalType\":\"struct FvmAddress\",\"name\":\"rawAddress\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint8\",\"name\":\"addrType\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"payload\",\"type\":\"bytes\",\"components\":[]}]}]},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"nonce\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"bytes4\",\"name\":\"method\",\"type\":\"bytes4\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"params\",\"type\":\"bytes\",\"components\":[]}]},{\"internalType\":\"bool\",\"name\":\"wrapped\",\"type\":\"bool\",\"components\":[]}]}]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"majorityPercentage\",\"outputs\":[{\"internalType\":\"uint64\",\"name\":\"\",\"type\":\"uint64\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"minStake\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"id\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"postbox\",\"outputs\":[{\"internalType\":\"struct StorableMsg\",\"name\":\"storableMsg\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"struct IPCAddress\",\"name\":\"from\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"struct SubnetID\",\"name\":\"subnetId\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint64\",\"name\":\"root\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"address[]\",\"name\":\"route\",\"type\":\"address[]\",\"components\":[]}]},{\"internalType\":\"struct FvmAddress\",\"name\":\"rawAddress\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint8\",\"name\":\"addrType\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"payload\",\"type\":\"bytes\",\"components\":[]}]}]},{\"internalType\":\"struct IPCAddress\",\"name\":\"to\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"struct SubnetID\",\"name\":\"subnetId\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint64\",\"name\":\"root\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"address[]\",\"name\":\"route\",\"type\":\"address[]\",\"components\":[]}]},{\"internalType\":\"struct FvmAddress\",\"name\":\"rawAddress\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint8\",\"name\":\"addrType\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"payload\",\"type\":\"bytes\",\"components\":[]}]}]},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"nonce\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"bytes4\",\"name\":\"method\",\"type\":\"bytes4\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"params\",\"type\":\"bytes\",\"components\":[]}]},{\"internalType\":\"bool\",\"name\":\"wrapped\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"h\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"subnets\",\"outputs\":[{\"internalType\":\"struct Subnet\",\"name\":\"subnet\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"stake\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"genesisEpoch\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"circSupply\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"topDownNonce\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"appliedBottomUpNonce\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"enum Status\",\"name\":\"status\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"struct SubnetID\",\"name\":\"id\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint64\",\"name\":\"root\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"address[]\",\"name\":\"route\",\"type\":\"address[]\",\"components\":[]}]},{\"internalType\":\"struct BottomUpCheckpoint\",\"name\":\"prevCheckpoint\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"struct SubnetID\",\"name\":\"source\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint64\",\"name\":\"root\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"address[]\",\"name\":\"route\",\"type\":\"address[]\",\"components\":[]}]},{\"internalType\":\"uint64\",\"name\":\"epoch\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"fee\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"struct CrossMsg[]\",\"name\":\"crossMsgs\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"struct StorableMsg\",\"name\":\"message\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"struct IPCAddress\",\"name\":\"from\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"struct SubnetID\",\"name\":\"subnetId\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint64\",\"name\":\"root\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"address[]\",\"name\":\"route\",\"type\":\"address[]\",\"components\":[]}]},{\"internalType\":\"struct FvmAddress\",\"name\":\"rawAddress\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint8\",\"name\":\"addrType\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"payload\",\"type\":\"bytes\",\"components\":[]}]}]},{\"internalType\":\"struct IPCAddress\",\"name\":\"to\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"struct SubnetID\",\"name\":\"subnetId\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint64\",\"name\":\"root\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"address[]\",\"name\":\"route\",\"type\":\"address[]\",\"components\":[]}]},{\"internalType\":\"struct FvmAddress\",\"name\":\"rawAddress\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint8\",\"name\":\"addrType\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"payload\",\"type\":\"bytes\",\"components\":[]}]}]},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"nonce\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"bytes4\",\"name\":\"method\",\"type\":\"bytes4\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"params\",\"type\":\"bytes\",\"components\":[]}]},{\"internalType\":\"bool\",\"name\":\"wrapped\",\"type\":\"bool\",\"components\":[]}]},{\"internalType\":\"struct ChildCheck[]\",\"name\":\"children\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"struct SubnetID\",\"name\":\"source\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint64\",\"name\":\"root\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"address[]\",\"name\":\"route\",\"type\":\"address[]\",\"components\":[]}]},{\"internalType\":\"bytes32[]\",\"name\":\"checks\",\"type\":\"bytes32[]\",\"components\":[]}]},{\"internalType\":\"bytes32\",\"name\":\"prevHash\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"proof\",\"type\":\"bytes\",\"components\":[]}]},{\"internalType\":\"struct CrossMsg[]\",\"name\":\"topDownMsgs\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"struct StorableMsg\",\"name\":\"message\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"struct IPCAddress\",\"name\":\"from\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"struct SubnetID\",\"name\":\"subnetId\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint64\",\"name\":\"root\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"address[]\",\"name\":\"route\",\"type\":\"address[]\",\"components\":[]}]},{\"internalType\":\"struct FvmAddress\",\"name\":\"rawAddress\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint8\",\"name\":\"addrType\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"payload\",\"type\":\"bytes\",\"components\":[]}]}]},{\"internalType\":\"struct IPCAddress\",\"name\":\"to\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"struct SubnetID\",\"name\":\"subnetId\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint64\",\"name\":\"root\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"address[]\",\"name\":\"route\",\"type\":\"address[]\",\"components\":[]}]},{\"internalType\":\"struct FvmAddress\",\"name\":\"rawAddress\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint8\",\"name\":\"addrType\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"payload\",\"type\":\"bytes\",\"components\":[]}]}]},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"nonce\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"bytes4\",\"name\":\"method\",\"type\":\"bytes4\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"params\",\"type\":\"bytes\",\"components\":[]}]},{\"internalType\":\"bool\",\"name\":\"wrapped\",\"type\":\"bool\",\"components\":[]}]}]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"topDownCheckPeriod\",\"outputs\":[{\"internalType\":\"uint64\",\"name\":\"\",\"type\":\"uint64\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"totalSubnets\",\"outputs\":[{\"internalType\":\"uint64\",\"name\":\"\",\"type\":\"uint64\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"totalWeight\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]}]";
    ///The parsed JSON ABI of the contract.
    pub static GATEWAYGETTERFACET_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(|| {
        ::ethers::core::utils::__serde_json::from_str(__ABI)
            .expect("ABI is always valid")
    });
    pub struct GatewayGetterFacet<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for GatewayGetterFacet<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for GatewayGetterFacet<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for GatewayGetterFacet<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for GatewayGetterFacet<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(GatewayGetterFacet)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> GatewayGetterFacet<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    GATEWAYGETTERFACET_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `appliedTopDownNonce` (0x8789f83b) function
        pub fn applied_top_down_nonce(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([135, 137, 248, 59], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `bottomUpCheckPeriod` (0x06c46853) function
        pub fn bottom_up_check_period(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([6, 196, 104, 83], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `bottomUpCheckpoints` (0x2cc14ea2) function
        pub fn bottom_up_checkpoints(
            &self,
            e: u64,
        ) -> ::ethers::contract::builders::ContractCall<M, BottomUpCheckpoint> {
            self.0
                .method_hash([44, 193, 78, 162], e)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `bottomUpNonce` (0x41b6a2e8) function
        pub fn bottom_up_nonce(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([65, 182, 162, 232], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `crossMsgFee` (0x24729425) function
        pub fn cross_msg_fee(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([36, 114, 148, 37], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `executableQueue` (0x10d500e1) function
        pub fn executable_queue(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, (u64, u64, u64)> {
            self.0
                .method_hash([16, 213, 0, 225], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getAppliedTopDownNonce` (0x9e530b57) function
        pub fn get_applied_top_down_nonce(
            &self,
            subnet_id: SubnetID,
        ) -> ::ethers::contract::builders::ContractCall<M, (bool, u64)> {
            self.0
                .method_hash([158, 83, 11, 87], (subnet_id,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getGenesisEpoch` (0x51392fc0) function
        pub fn get_genesis_epoch(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([81, 57, 47, 192], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getNetworkName` (0x94074b03) function
        pub fn get_network_name(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, SubnetID> {
            self.0
                .method_hash([148, 7, 75, 3], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getSubnet` (0xc66c66a1) function
        pub fn get_subnet(
            &self,
            subnet_id: SubnetID,
        ) -> ::ethers::contract::builders::ContractCall<M, (bool, Subnet)> {
            self.0
                .method_hash([198, 108, 102, 161], (subnet_id,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getSubnetTopDownMsg` (0x0ea746f2) function
        pub fn get_subnet_top_down_msg(
            &self,
            subnet_id: SubnetID,
            index: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, CrossMsg> {
            self.0
                .method_hash([14, 167, 70, 242], (subnet_id, index))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getSubnetTopDownMsgsLength` (0x9d3070b5) function
        pub fn get_subnet_top_down_msgs_length(
            &self,
            subnet_id: SubnetID,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([157, 48, 112, 181], (subnet_id,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getTopDownMsgs` (0x13549315) function
        pub fn get_top_down_msgs(
            &self,
            subnet_id: SubnetID,
            from_nonce: u64,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<CrossMsg>> {
            self.0
                .method_hash([19, 84, 147, 21], (subnet_id, from_nonce))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `hasValidatorVotedForSubmission` (0x66d7bbbc) function
        pub fn has_validator_voted_for_submission(
            &self,
            epoch: u64,
            submitter: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([102, 215, 187, 188], (epoch, submitter))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `initialized` (0x158ef93e) function
        pub fn initialized(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([21, 142, 249, 62], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `lastVotingExecutedEpoch` (0xad81e244) function
        pub fn last_voting_executed_epoch(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([173, 129, 226, 68], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `listSubnets` (0x5d029685) function
        pub fn list_subnets(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<Subnet>> {
            self.0
                .method_hash([93, 2, 150, 133], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `majorityPercentage` (0x599c7bd1) function
        pub fn majority_percentage(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([89, 156, 123, 209], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `minStake` (0x375b3c0a) function
        pub fn min_stake(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([55, 91, 60, 10], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `postbox` (0x8cfd78e7) function
        pub fn postbox(
            &self,
            id: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, (StorableMsg, bool)> {
            self.0
                .method_hash([140, 253, 120, 231], id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `subnets` (0x02e30f9a) function
        pub fn subnets(
            &self,
            h: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, Subnet> {
            self.0
                .method_hash([2, 227, 15, 154], h)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `topDownCheckPeriod` (0x7d9740f4) function
        pub fn top_down_check_period(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([125, 151, 64, 244], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `totalSubnets` (0xa2b67158) function
        pub fn total_subnets(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([162, 182, 113, 88], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `totalWeight` (0x96c82e57) function
        pub fn total_weight(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([150, 200, 46, 87], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for GatewayGetterFacet<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `appliedTopDownNonce` function with signature `appliedTopDownNonce()` and selector `0x8789f83b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "appliedTopDownNonce", abi = "appliedTopDownNonce()")]
    pub struct AppliedTopDownNonceCall;
    ///Container type for all input parameters for the `bottomUpCheckPeriod` function with signature `bottomUpCheckPeriod()` and selector `0x06c46853`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "bottomUpCheckPeriod", abi = "bottomUpCheckPeriod()")]
    pub struct BottomUpCheckPeriodCall;
    ///Container type for all input parameters for the `bottomUpCheckpoints` function with signature `bottomUpCheckpoints(uint64)` and selector `0x2cc14ea2`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "bottomUpCheckpoints", abi = "bottomUpCheckpoints(uint64)")]
    pub struct BottomUpCheckpointsCall {
        pub e: u64,
    }
    ///Container type for all input parameters for the `bottomUpNonce` function with signature `bottomUpNonce()` and selector `0x41b6a2e8`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "bottomUpNonce", abi = "bottomUpNonce()")]
    pub struct BottomUpNonceCall;
    ///Container type for all input parameters for the `crossMsgFee` function with signature `crossMsgFee()` and selector `0x24729425`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "crossMsgFee", abi = "crossMsgFee()")]
    pub struct CrossMsgFeeCall;
    ///Container type for all input parameters for the `executableQueue` function with signature `executableQueue()` and selector `0x10d500e1`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "executableQueue", abi = "executableQueue()")]
    pub struct ExecutableQueueCall;
    ///Container type for all input parameters for the `getAppliedTopDownNonce` function with signature `getAppliedTopDownNonce((uint64,address[]))` and selector `0x9e530b57`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "getAppliedTopDownNonce",
        abi = "getAppliedTopDownNonce((uint64,address[]))"
    )]
    pub struct GetAppliedTopDownNonceCall {
        pub subnet_id: SubnetID,
    }
    ///Container type for all input parameters for the `getGenesisEpoch` function with signature `getGenesisEpoch()` and selector `0x51392fc0`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getGenesisEpoch", abi = "getGenesisEpoch()")]
    pub struct GetGenesisEpochCall;
    ///Container type for all input parameters for the `getNetworkName` function with signature `getNetworkName()` and selector `0x94074b03`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getNetworkName", abi = "getNetworkName()")]
    pub struct GetNetworkNameCall;
    ///Container type for all input parameters for the `getSubnet` function with signature `getSubnet((uint64,address[]))` and selector `0xc66c66a1`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getSubnet", abi = "getSubnet((uint64,address[]))")]
    pub struct GetSubnetCall {
        pub subnet_id: SubnetID,
    }
    ///Container type for all input parameters for the `getSubnetTopDownMsg` function with signature `getSubnetTopDownMsg((uint64,address[]),uint256)` and selector `0x0ea746f2`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "getSubnetTopDownMsg",
        abi = "getSubnetTopDownMsg((uint64,address[]),uint256)"
    )]
    pub struct GetSubnetTopDownMsgCall {
        pub subnet_id: SubnetID,
        pub index: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getSubnetTopDownMsgsLength` function with signature `getSubnetTopDownMsgsLength((uint64,address[]))` and selector `0x9d3070b5`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "getSubnetTopDownMsgsLength",
        abi = "getSubnetTopDownMsgsLength((uint64,address[]))"
    )]
    pub struct GetSubnetTopDownMsgsLengthCall {
        pub subnet_id: SubnetID,
    }
    ///Container type for all input parameters for the `getTopDownMsgs` function with signature `getTopDownMsgs((uint64,address[]),uint64)` and selector `0x13549315`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "getTopDownMsgs",
        abi = "getTopDownMsgs((uint64,address[]),uint64)"
    )]
    pub struct GetTopDownMsgsCall {
        pub subnet_id: SubnetID,
        pub from_nonce: u64,
    }
    ///Container type for all input parameters for the `hasValidatorVotedForSubmission` function with signature `hasValidatorVotedForSubmission(uint64,address)` and selector `0x66d7bbbc`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "hasValidatorVotedForSubmission",
        abi = "hasValidatorVotedForSubmission(uint64,address)"
    )]
    pub struct HasValidatorVotedForSubmissionCall {
        pub epoch: u64,
        pub submitter: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `initialized` function with signature `initialized()` and selector `0x158ef93e`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "initialized", abi = "initialized()")]
    pub struct InitializedCall;
    ///Container type for all input parameters for the `lastVotingExecutedEpoch` function with signature `lastVotingExecutedEpoch()` and selector `0xad81e244`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "lastVotingExecutedEpoch", abi = "lastVotingExecutedEpoch()")]
    pub struct LastVotingExecutedEpochCall;
    ///Container type for all input parameters for the `listSubnets` function with signature `listSubnets()` and selector `0x5d029685`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "listSubnets", abi = "listSubnets()")]
    pub struct ListSubnetsCall;
    ///Container type for all input parameters for the `majorityPercentage` function with signature `majorityPercentage()` and selector `0x599c7bd1`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "majorityPercentage", abi = "majorityPercentage()")]
    pub struct MajorityPercentageCall;
    ///Container type for all input parameters for the `minStake` function with signature `minStake()` and selector `0x375b3c0a`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "minStake", abi = "minStake()")]
    pub struct MinStakeCall;
    ///Container type for all input parameters for the `postbox` function with signature `postbox(bytes32)` and selector `0x8cfd78e7`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "postbox", abi = "postbox(bytes32)")]
    pub struct PostboxCall {
        pub id: [u8; 32],
    }
    ///Container type for all input parameters for the `subnets` function with signature `subnets(bytes32)` and selector `0x02e30f9a`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "subnets", abi = "subnets(bytes32)")]
    pub struct SubnetsCall {
        pub h: [u8; 32],
    }
    ///Container type for all input parameters for the `topDownCheckPeriod` function with signature `topDownCheckPeriod()` and selector `0x7d9740f4`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "topDownCheckPeriod", abi = "topDownCheckPeriod()")]
    pub struct TopDownCheckPeriodCall;
    ///Container type for all input parameters for the `totalSubnets` function with signature `totalSubnets()` and selector `0xa2b67158`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "totalSubnets", abi = "totalSubnets()")]
    pub struct TotalSubnetsCall;
    ///Container type for all input parameters for the `totalWeight` function with signature `totalWeight()` and selector `0x96c82e57`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "totalWeight", abi = "totalWeight()")]
    pub struct TotalWeightCall;
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum GatewayGetterFacetCalls {
        AppliedTopDownNonce(AppliedTopDownNonceCall),
        BottomUpCheckPeriod(BottomUpCheckPeriodCall),
        BottomUpCheckpoints(BottomUpCheckpointsCall),
        BottomUpNonce(BottomUpNonceCall),
        CrossMsgFee(CrossMsgFeeCall),
        ExecutableQueue(ExecutableQueueCall),
        GetAppliedTopDownNonce(GetAppliedTopDownNonceCall),
        GetGenesisEpoch(GetGenesisEpochCall),
        GetNetworkName(GetNetworkNameCall),
        GetSubnet(GetSubnetCall),
        GetSubnetTopDownMsg(GetSubnetTopDownMsgCall),
        GetSubnetTopDownMsgsLength(GetSubnetTopDownMsgsLengthCall),
        GetTopDownMsgs(GetTopDownMsgsCall),
        HasValidatorVotedForSubmission(HasValidatorVotedForSubmissionCall),
        Initialized(InitializedCall),
        LastVotingExecutedEpoch(LastVotingExecutedEpochCall),
        ListSubnets(ListSubnetsCall),
        MajorityPercentage(MajorityPercentageCall),
        MinStake(MinStakeCall),
        Postbox(PostboxCall),
        Subnets(SubnetsCall),
        TopDownCheckPeriod(TopDownCheckPeriodCall),
        TotalSubnets(TotalSubnetsCall),
        TotalWeight(TotalWeightCall),
    }
    impl ::ethers::core::abi::AbiDecode for GatewayGetterFacetCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <AppliedTopDownNonceCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::AppliedTopDownNonce(decoded));
            }
            if let Ok(decoded)
                = <BottomUpCheckPeriodCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::BottomUpCheckPeriod(decoded));
            }
            if let Ok(decoded)
                = <BottomUpCheckpointsCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::BottomUpCheckpoints(decoded));
            }
            if let Ok(decoded)
                = <BottomUpNonceCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::BottomUpNonce(decoded));
            }
            if let Ok(decoded)
                = <CrossMsgFeeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::CrossMsgFee(decoded));
            }
            if let Ok(decoded)
                = <ExecutableQueueCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ExecutableQueue(decoded));
            }
            if let Ok(decoded)
                = <GetAppliedTopDownNonceCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetAppliedTopDownNonce(decoded));
            }
            if let Ok(decoded)
                = <GetGenesisEpochCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetGenesisEpoch(decoded));
            }
            if let Ok(decoded)
                = <GetNetworkNameCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetNetworkName(decoded));
            }
            if let Ok(decoded)
                = <GetSubnetCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetSubnet(decoded));
            }
            if let Ok(decoded)
                = <GetSubnetTopDownMsgCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetSubnetTopDownMsg(decoded));
            }
            if let Ok(decoded)
                = <GetSubnetTopDownMsgsLengthCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetSubnetTopDownMsgsLength(decoded));
            }
            if let Ok(decoded)
                = <GetTopDownMsgsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetTopDownMsgs(decoded));
            }
            if let Ok(decoded)
                = <HasValidatorVotedForSubmissionCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::HasValidatorVotedForSubmission(decoded));
            }
            if let Ok(decoded)
                = <InitializedCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Initialized(decoded));
            }
            if let Ok(decoded)
                = <LastVotingExecutedEpochCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::LastVotingExecutedEpoch(decoded));
            }
            if let Ok(decoded)
                = <ListSubnetsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ListSubnets(decoded));
            }
            if let Ok(decoded)
                = <MajorityPercentageCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::MajorityPercentage(decoded));
            }
            if let Ok(decoded)
                = <MinStakeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::MinStake(decoded));
            }
            if let Ok(decoded)
                = <PostboxCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Postbox(decoded));
            }
            if let Ok(decoded)
                = <SubnetsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Subnets(decoded));
            }
            if let Ok(decoded)
                = <TopDownCheckPeriodCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::TopDownCheckPeriod(decoded));
            }
            if let Ok(decoded)
                = <TotalSubnetsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::TotalSubnets(decoded));
            }
            if let Ok(decoded)
                = <TotalWeightCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::TotalWeight(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for GatewayGetterFacetCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AppliedTopDownNonce(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BottomUpCheckPeriod(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BottomUpCheckpoints(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BottomUpNonce(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CrossMsgFee(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExecutableQueue(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetAppliedTopDownNonce(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetGenesisEpoch(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetNetworkName(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetSubnet(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetSubnetTopDownMsg(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetSubnetTopDownMsgsLength(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetTopDownMsgs(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::HasValidatorVotedForSubmission(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Initialized(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LastVotingExecutedEpoch(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ListSubnets(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MajorityPercentage(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MinStake(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Postbox(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Subnets(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TopDownCheckPeriod(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TotalSubnets(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TotalWeight(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for GatewayGetterFacetCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AppliedTopDownNonce(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::BottomUpCheckPeriod(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::BottomUpCheckpoints(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::BottomUpNonce(element) => ::core::fmt::Display::fmt(element, f),
                Self::CrossMsgFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExecutableQueue(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetAppliedTopDownNonce(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetGenesisEpoch(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetNetworkName(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetSubnet(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetSubnetTopDownMsg(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetSubnetTopDownMsgsLength(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetTopDownMsgs(element) => ::core::fmt::Display::fmt(element, f),
                Self::HasValidatorVotedForSubmission(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Initialized(element) => ::core::fmt::Display::fmt(element, f),
                Self::LastVotingExecutedEpoch(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ListSubnets(element) => ::core::fmt::Display::fmt(element, f),
                Self::MajorityPercentage(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::MinStake(element) => ::core::fmt::Display::fmt(element, f),
                Self::Postbox(element) => ::core::fmt::Display::fmt(element, f),
                Self::Subnets(element) => ::core::fmt::Display::fmt(element, f),
                Self::TopDownCheckPeriod(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TotalSubnets(element) => ::core::fmt::Display::fmt(element, f),
                Self::TotalWeight(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AppliedTopDownNonceCall> for GatewayGetterFacetCalls {
        fn from(value: AppliedTopDownNonceCall) -> Self {
            Self::AppliedTopDownNonce(value)
        }
    }
    impl ::core::convert::From<BottomUpCheckPeriodCall> for GatewayGetterFacetCalls {
        fn from(value: BottomUpCheckPeriodCall) -> Self {
            Self::BottomUpCheckPeriod(value)
        }
    }
    impl ::core::convert::From<BottomUpCheckpointsCall> for GatewayGetterFacetCalls {
        fn from(value: BottomUpCheckpointsCall) -> Self {
            Self::BottomUpCheckpoints(value)
        }
    }
    impl ::core::convert::From<BottomUpNonceCall> for GatewayGetterFacetCalls {
        fn from(value: BottomUpNonceCall) -> Self {
            Self::BottomUpNonce(value)
        }
    }
    impl ::core::convert::From<CrossMsgFeeCall> for GatewayGetterFacetCalls {
        fn from(value: CrossMsgFeeCall) -> Self {
            Self::CrossMsgFee(value)
        }
    }
    impl ::core::convert::From<ExecutableQueueCall> for GatewayGetterFacetCalls {
        fn from(value: ExecutableQueueCall) -> Self {
            Self::ExecutableQueue(value)
        }
    }
    impl ::core::convert::From<GetAppliedTopDownNonceCall> for GatewayGetterFacetCalls {
        fn from(value: GetAppliedTopDownNonceCall) -> Self {
            Self::GetAppliedTopDownNonce(value)
        }
    }
    impl ::core::convert::From<GetGenesisEpochCall> for GatewayGetterFacetCalls {
        fn from(value: GetGenesisEpochCall) -> Self {
            Self::GetGenesisEpoch(value)
        }
    }
    impl ::core::convert::From<GetNetworkNameCall> for GatewayGetterFacetCalls {
        fn from(value: GetNetworkNameCall) -> Self {
            Self::GetNetworkName(value)
        }
    }
    impl ::core::convert::From<GetSubnetCall> for GatewayGetterFacetCalls {
        fn from(value: GetSubnetCall) -> Self {
            Self::GetSubnet(value)
        }
    }
    impl ::core::convert::From<GetSubnetTopDownMsgCall> for GatewayGetterFacetCalls {
        fn from(value: GetSubnetTopDownMsgCall) -> Self {
            Self::GetSubnetTopDownMsg(value)
        }
    }
    impl ::core::convert::From<GetSubnetTopDownMsgsLengthCall>
    for GatewayGetterFacetCalls {
        fn from(value: GetSubnetTopDownMsgsLengthCall) -> Self {
            Self::GetSubnetTopDownMsgsLength(value)
        }
    }
    impl ::core::convert::From<GetTopDownMsgsCall> for GatewayGetterFacetCalls {
        fn from(value: GetTopDownMsgsCall) -> Self {
            Self::GetTopDownMsgs(value)
        }
    }
    impl ::core::convert::From<HasValidatorVotedForSubmissionCall>
    for GatewayGetterFacetCalls {
        fn from(value: HasValidatorVotedForSubmissionCall) -> Self {
            Self::HasValidatorVotedForSubmission(value)
        }
    }
    impl ::core::convert::From<InitializedCall> for GatewayGetterFacetCalls {
        fn from(value: InitializedCall) -> Self {
            Self::Initialized(value)
        }
    }
    impl ::core::convert::From<LastVotingExecutedEpochCall> for GatewayGetterFacetCalls {
        fn from(value: LastVotingExecutedEpochCall) -> Self {
            Self::LastVotingExecutedEpoch(value)
        }
    }
    impl ::core::convert::From<ListSubnetsCall> for GatewayGetterFacetCalls {
        fn from(value: ListSubnetsCall) -> Self {
            Self::ListSubnets(value)
        }
    }
    impl ::core::convert::From<MajorityPercentageCall> for GatewayGetterFacetCalls {
        fn from(value: MajorityPercentageCall) -> Self {
            Self::MajorityPercentage(value)
        }
    }
    impl ::core::convert::From<MinStakeCall> for GatewayGetterFacetCalls {
        fn from(value: MinStakeCall) -> Self {
            Self::MinStake(value)
        }
    }
    impl ::core::convert::From<PostboxCall> for GatewayGetterFacetCalls {
        fn from(value: PostboxCall) -> Self {
            Self::Postbox(value)
        }
    }
    impl ::core::convert::From<SubnetsCall> for GatewayGetterFacetCalls {
        fn from(value: SubnetsCall) -> Self {
            Self::Subnets(value)
        }
    }
    impl ::core::convert::From<TopDownCheckPeriodCall> for GatewayGetterFacetCalls {
        fn from(value: TopDownCheckPeriodCall) -> Self {
            Self::TopDownCheckPeriod(value)
        }
    }
    impl ::core::convert::From<TotalSubnetsCall> for GatewayGetterFacetCalls {
        fn from(value: TotalSubnetsCall) -> Self {
            Self::TotalSubnets(value)
        }
    }
    impl ::core::convert::From<TotalWeightCall> for GatewayGetterFacetCalls {
        fn from(value: TotalWeightCall) -> Self {
            Self::TotalWeight(value)
        }
    }
    ///Container type for all return fields from the `appliedTopDownNonce` function with signature `appliedTopDownNonce()` and selector `0x8789f83b`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct AppliedTopDownNonceReturn(pub u64);
    ///Container type for all return fields from the `bottomUpCheckPeriod` function with signature `bottomUpCheckPeriod()` and selector `0x06c46853`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct BottomUpCheckPeriodReturn(pub u64);
    ///Container type for all return fields from the `bottomUpCheckpoints` function with signature `bottomUpCheckpoints(uint64)` and selector `0x2cc14ea2`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct BottomUpCheckpointsReturn(pub BottomUpCheckpoint);
    ///Container type for all return fields from the `bottomUpNonce` function with signature `bottomUpNonce()` and selector `0x41b6a2e8`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct BottomUpNonceReturn(pub u64);
    ///Container type for all return fields from the `crossMsgFee` function with signature `crossMsgFee()` and selector `0x24729425`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct CrossMsgFeeReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `executableQueue` function with signature `executableQueue()` and selector `0x10d500e1`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct ExecutableQueueReturn(pub u64, pub u64, pub u64);
    ///Container type for all return fields from the `getAppliedTopDownNonce` function with signature `getAppliedTopDownNonce((uint64,address[]))` and selector `0x9e530b57`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct GetAppliedTopDownNonceReturn(pub bool, pub u64);
    ///Container type for all return fields from the `getGenesisEpoch` function with signature `getGenesisEpoch()` and selector `0x51392fc0`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct GetGenesisEpochReturn(pub u64);
    ///Container type for all return fields from the `getNetworkName` function with signature `getNetworkName()` and selector `0x94074b03`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct GetNetworkNameReturn(pub SubnetID);
    ///Container type for all return fields from the `getSubnet` function with signature `getSubnet((uint64,address[]))` and selector `0xc66c66a1`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct GetSubnetReturn(pub bool, pub Subnet);
    ///Container type for all return fields from the `getSubnetTopDownMsg` function with signature `getSubnetTopDownMsg((uint64,address[]),uint256)` and selector `0x0ea746f2`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct GetSubnetTopDownMsgReturn(pub CrossMsg);
    ///Container type for all return fields from the `getSubnetTopDownMsgsLength` function with signature `getSubnetTopDownMsgsLength((uint64,address[]))` and selector `0x9d3070b5`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct GetSubnetTopDownMsgsLengthReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getTopDownMsgs` function with signature `getTopDownMsgs((uint64,address[]),uint64)` and selector `0x13549315`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct GetTopDownMsgsReturn(pub ::std::vec::Vec<CrossMsg>);
    ///Container type for all return fields from the `hasValidatorVotedForSubmission` function with signature `hasValidatorVotedForSubmission(uint64,address)` and selector `0x66d7bbbc`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct HasValidatorVotedForSubmissionReturn(pub bool);
    ///Container type for all return fields from the `initialized` function with signature `initialized()` and selector `0x158ef93e`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct InitializedReturn(pub bool);
    ///Container type for all return fields from the `lastVotingExecutedEpoch` function with signature `lastVotingExecutedEpoch()` and selector `0xad81e244`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct LastVotingExecutedEpochReturn(pub u64);
    ///Container type for all return fields from the `listSubnets` function with signature `listSubnets()` and selector `0x5d029685`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct ListSubnetsReturn(pub ::std::vec::Vec<Subnet>);
    ///Container type for all return fields from the `majorityPercentage` function with signature `majorityPercentage()` and selector `0x599c7bd1`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct MajorityPercentageReturn(pub u64);
    ///Container type for all return fields from the `minStake` function with signature `minStake()` and selector `0x375b3c0a`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct MinStakeReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `postbox` function with signature `postbox(bytes32)` and selector `0x8cfd78e7`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct PostboxReturn {
        pub storable_msg: StorableMsg,
        pub wrapped: bool,
    }
    ///Container type for all return fields from the `subnets` function with signature `subnets(bytes32)` and selector `0x02e30f9a`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct SubnetsReturn {
        pub subnet: Subnet,
    }
    ///Container type for all return fields from the `topDownCheckPeriod` function with signature `topDownCheckPeriod()` and selector `0x7d9740f4`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct TopDownCheckPeriodReturn(pub u64);
    ///Container type for all return fields from the `totalSubnets` function with signature `totalSubnets()` and selector `0xa2b67158`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct TotalSubnetsReturn(pub u64);
    ///Container type for all return fields from the `totalWeight` function with signature `totalWeight()` and selector `0x96c82e57`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct TotalWeightReturn(pub ::ethers::core::types::U256);
    ///`BottomUpCheckpoint((uint64,address[]),uint64,uint256,((((uint64,address[]),(uint8,bytes)),((uint64,address[]),(uint8,bytes)),uint256,uint64,bytes4,bytes),bool)[],((uint64,address[]),bytes32[])[],bytes32,bytes)`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct BottomUpCheckpoint {
        pub source: SubnetID,
        pub epoch: u64,
        pub fee: ::ethers::core::types::U256,
        pub cross_msgs: ::std::vec::Vec<CrossMsg>,
        pub children: ::std::vec::Vec<ChildCheck>,
        pub prev_hash: [u8; 32],
        pub proof: ::ethers::core::types::Bytes,
    }
    ///`ChildCheck((uint64,address[]),bytes32[])`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct ChildCheck {
        pub source: SubnetID,
        pub checks: ::std::vec::Vec<[u8; 32]>,
    }
    ///`CrossMsg((((uint64,address[]),(uint8,bytes)),((uint64,address[]),(uint8,bytes)),uint256,uint64,bytes4,bytes),bool)`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct CrossMsg {
        pub message: StorableMsg,
        pub wrapped: bool,
    }
    ///`FvmAddress(uint8,bytes)`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct FvmAddress {
        pub addr_type: u8,
        pub payload: ::ethers::core::types::Bytes,
    }
    ///`Ipcaddress((uint64,address[]),(uint8,bytes))`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct Ipcaddress {
        pub subnet_id: SubnetID,
        pub raw_address: FvmAddress,
    }
    ///`StorableMsg(((uint64,address[]),(uint8,bytes)),((uint64,address[]),(uint8,bytes)),uint256,uint64,bytes4,bytes)`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct StorableMsg {
        pub from: Ipcaddress,
        pub to: Ipcaddress,
        pub value: ::ethers::core::types::U256,
        pub nonce: u64,
        pub method: [u8; 4],
        pub params: ::ethers::core::types::Bytes,
    }
    ///`Subnet(uint256,uint256,uint256,uint64,uint64,uint8,(uint64,address[]),((uint64,address[]),uint64,uint256,((((uint64,address[]),(uint8,bytes)),((uint64,address[]),(uint8,bytes)),uint256,uint64,bytes4,bytes),bool)[],((uint64,address[]),bytes32[])[],bytes32,bytes),((((uint64,address[]),(uint8,bytes)),((uint64,address[]),(uint8,bytes)),uint256,uint64,bytes4,bytes),bool)[])`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct Subnet {
        pub stake: ::ethers::core::types::U256,
        pub genesis_epoch: ::ethers::core::types::U256,
        pub circ_supply: ::ethers::core::types::U256,
        pub top_down_nonce: u64,
        pub applied_bottom_up_nonce: u64,
        pub status: u8,
        pub id: SubnetID,
        pub prev_checkpoint: BottomUpCheckpoint,
        pub top_down_msgs: ::std::vec::Vec<CrossMsg>,
    }
    ///`SubnetID(uint64,address[])`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct SubnetID {
        pub root: u64,
        pub route: ::std::vec::Vec<::ethers::core::types::Address>,
    }
}