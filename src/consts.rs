use std::mem;

use buffering::copy::{StreamReadBuffer,StreamWriteBuffer};
use libc;

use Nl;
use err::{SerError,DeError};

macro_rules! impl_var {
    ( $name:ident, $ty:ty, $var_def:ident => $val_def:expr,
      $( $var:ident => $val:expr ),* ) => (
        /// Enum representing C constants for netlink packets
        #[derive(Clone,Debug,Eq,PartialEq)]
        pub enum $name {
            #[allow(missing_docs)]
            $var_def,
            $(
                #[allow(missing_docs)]
                $var,
            )*
            /// Variant that signifies an invalid value while deserializing
            UnrecognizedVariant($ty),
        }

        impl From<$ty> for $name {
            fn from(v: $ty) -> Self {
                match v {
                    i if i == $val_def => $name::$var_def,
                    $( i if i == $val => $name::$var, )*
                    i => $name::UnrecognizedVariant(i)
                }
            }
        }

        impl From<$name> for $ty {
            fn from(v: $name) -> Self {
                match v {
                    $name::$var_def => $val_def,
                    $( $name::$var => $val, )*
                    $name::UnrecognizedVariant(i) => i,
                }
            }
        }

        impl Nl for $name {
            type SerIn = ();
            type DeIn = ();

            fn serialize(&self, mem: &mut StreamWriteBuffer) -> Result<(), SerError> {
                let v: $ty = self.clone().into();
                v.serialize(mem)
            }

            fn deserialize<T>(mem: &mut StreamReadBuffer<T>) -> Result<Self, DeError>
                    where T: AsRef<[u8]> {
                let v: $ty = Nl::deserialize(mem)?;
                Ok(v.into())
            }

            fn size(&self) -> usize {
                mem::size_of::<$ty>()
            }
        }
    );
}

/// Reimplementation of alignto macro in C
pub fn alignto(len: usize) -> usize {
    (len + libc::NLA_ALIGNTO as usize - 1) & !(libc::NLA_ALIGNTO as usize - 1)
}

/// Address families for sockets
impl_var!(AddrFamily, libc::c_int,
    UnixOrLocal => libc::AF_UNIX,
    Inet => libc::AF_INET,
    Inet6 => libc::AF_INET6,
    Ipx => libc::AF_IPX,
    Netlink => libc::AF_NETLINK,
    X25 => libc::AF_X25,
    AX25 => libc::AF_AX25,
    Atmpvc => libc::AF_ATMPVC,
    Appletalk => libc::AF_APPLETALK,
    Packet => libc::AF_PACKET,
    Alg => libc::AF_ALG
);

/// Interface types
impl_var!(Arphrd, libc::c_ushort,
    Netrom => libc::ARPHRD_NETROM,
    Ether => libc::ARPHRD_ETHER
);

/// Values for `nl_family` in `NlSocket`
impl_var!(NlFamily, libc::c_int,
    Route => libc::NETLINK_ROUTE,
    Unused => libc::NETLINK_UNUSED,
    Usersock => libc::NETLINK_USERSOCK,
    Firewall => libc::NETLINK_FIREWALL,
    SockOrInetDiag => libc::NETLINK_SOCK_DIAG,
    Nflog => libc::NETLINK_NFLOG,
    Xfrm => libc::NETLINK_XFRM,
    Selinux => libc::NETLINK_SELINUX,
    Iscsi => libc::NETLINK_ISCSI,
    Audit => libc::NETLINK_AUDIT,
    FibLookup => libc::NETLINK_FIB_LOOKUP,
    Connector => libc::NETLINK_CONNECTOR,
    Netfilter => libc::NETLINK_NETFILTER,
    Ip6Fw => libc::NETLINK_IP6_FW,
    Dnrtmsg => libc::NETLINK_DNRTMSG,
    KobjectUevent => libc::NETLINK_KOBJECT_UEVENT,
    Generic => libc::NETLINK_GENERIC,
    Scsitransport => libc::NETLINK_SCSITRANSPORT,
    Ecryptfs => libc::NETLINK_ECRYPTFS,
    Rdma => libc::NETLINK_RDMA,
    Crypto => libc::NETLINK_CRYPTO
);

/// Values for `nl_type` in `NlHdr`
impl_var!(Nlmsg, u16,
    Noop => libc::NLMSG_NOOP as u16,
    Error => libc::NLMSG_ERROR as u16,
    Done => libc::NLMSG_DONE as u16,
    Overrun => libc::NLMSG_OVERRUN as u16
);

/// Values for `nl_flags` in `NlHdr`
impl_var!(NlmF, u16,
    Request => libc::NLM_F_REQUEST as u16,
    Multi => libc::NLM_F_MULTI as u16,
    Ack => libc::NLM_F_ACK as u16,
    Echo => libc::NLM_F_ECHO as u16,
    DumpIntr => libc::NLM_F_DUMP_INTR as u16,
    DumpFiltered => libc::NLM_F_DUMP_FILTERED as u16,
    Root => libc::NLM_F_ROOT as u16,
    Match => libc::NLM_F_MATCH as u16,
    Atomic => libc::NLM_F_ATOMIC as u16,
    Dump => libc::NLM_F_DUMP as u16,
    Replace => libc::NLM_F_REPLACE as u16,
    Excl => libc::NLM_F_EXCL as u16,
    Create => libc::NLM_F_CREATE as u16,
    Append => libc::NLM_F_APPEND as u16
);

impl_var!(GenlId, u16,
    Ctrl => libc::GENL_ID_CTRL as u16,
    VfsDquot => libc::GENL_ID_VFS_DQUOT as u16,
    Pmcraid => libc::GENL_ID_PMCRAID as u16 
);

/// Values for `cmd` in `GenlHdr`
impl_var!(CtrlCmd, u8,
    Unspec => libc::CTRL_CMD_UNSPEC as u8,
    Newfamily => libc::CTRL_CMD_NEWFAMILY as u8,
    Delfamily => libc::CTRL_CMD_DELFAMILY as u8,
    Getfamily => libc::CTRL_CMD_GETFAMILY as u8,
    Newops => libc::CTRL_CMD_NEWOPS as u8,
    Delops => libc::CTRL_CMD_DELOPS as u8,
    Getops => libc::CTRL_CMD_GETOPS as u8,
    NewmcastGrp => libc::CTRL_CMD_NEWMCAST_GRP as u8,
    DelmcastGrp => libc::CTRL_CMD_DELMCAST_GRP as u8,
    GetmcastGrp => libc::CTRL_CMD_GETMCAST_GRP as u8
);

/// Values for `nla_type` in `NlaAttrHdr`
impl_var!(CtrlAttr, u16,
    Unspec => libc::CTRL_ATTR_UNSPEC as u16,
    FamilyId => libc::CTRL_ATTR_FAMILY_ID as u16,
    FamilyName => libc::CTRL_ATTR_FAMILY_NAME as u16,
    Version => libc::CTRL_ATTR_VERSION as u16,
    Hdrsize => libc::CTRL_ATTR_HDRSIZE as u16,
    Maxattr => libc::CTRL_ATTR_MAXATTR as u16,
    Ops => libc::CTRL_ATTR_OPS as u16,
    McastGroups => libc::CTRL_ATTR_MCAST_GROUPS as u16
);

/// Values for `nla_type` in `NlaAttrHdr`
impl_var!(CtrlAttrMcastGrp, u16,
    Unspec => libc::CTRL_ATTR_MCAST_GRP_UNSPEC as u16,
    Name => libc::CTRL_ATTR_MCAST_GRP_NAME as u16,
    Id => libc::CTRL_ATTR_MCAST_GRP_ID as u16
);
