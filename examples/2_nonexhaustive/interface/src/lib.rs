use abi_stable::{
    declare_root_module_statics,
    rvec,
    external_types::{RawValueBox, RawValueRef},
    library::RootModule,
    nonexhaustive_enum::{DeserializeEnum, NonExhaustiveFor, SerializeEnum},
    package_version_strings, sabi_trait,
    sabi_types::VersionStrings,
    std_types::{RBox, RBoxError, RResult, RStr, RString, RVec},
    StableAbi,
};

use serde::{Deserialize, Serialize};

/// Represents United States cents.
#[repr(transparent)]
#[derive(StableAbi, Debug, Clone, Copy, PartialEq, Deserialize, Serialize)]
pub struct Cents {
    pub cents: u64,
}

/// The unique identifier of an item that is/was in the catalogue,
/// which are never reused for a different item.
#[repr(transparent)]
#[derive(StableAbi, Debug, Clone, Copy, PartialEq, Deserialize, Serialize)]
pub struct ItemId {
    #[doc(hidden)]
    pub id: usize,
}

///////////////////////////////////////////////////////////////////////////////

/// The parameters of `Shop::run_command`.
///
/// Every variant of this enum corresponds to a variant of `ReturnVal`.
#[non_exhaustive]
#[repr(u8)]
#[derive(StableAbi, Debug, Clone, PartialEq, Deserialize, Serialize)]
#[sabi(kind(WithNonExhaustive(
    size = [usize;8],
    traits(Send, Sync, Debug, Clone, PartialEq, Serialize, Deserialize),
    assert_nonexhaustive = Command,
)))]
pub enum Command {
    /// `#[sabi(with_boxed_constructor)]` tells the `StableAbi` derive macro to
    /// generate the `fn CreateItem_NE(ParamCreateItem)->ReturnVal_NE` associated function.
    #[sabi(with_boxed_constructor)]
    CreateItem(RBox<ParamCreateItem>),
    DeleteItem {
        id: ItemId,
    },
    AddItem {
        id: ItemId,
        count: u32,
    },
    RemoveItem {
        id: ItemId,
        count: u32,
    },

    /// This variant was added in the 1.1 version of the library.
    #[cfg(feature = "v1_1")]
    RenameItem {
        id: ItemId,
        new_name: RString,
    },

    /// This variant was added in the 1.1 version of the library.
    ///
    /// `#[sabi(with_constructor)]` tells the `StableAbi` derive macro to
    /// generate the `fn Many_NE(RVec<Command_NE>)->Command_NE` associated function.
    #[cfg(feature = "v1_1")]
    #[sabi(with_constructor)]
    Many {
        list: RVec<Command_NE>,
    },
}

/*
//This was generated by the StableAbi derive macro on Command.
pub type Command_NE=
    NonExhaustive<
        Command,
        Command_Storage,
        Command_Interface,
    >;
*/

#[repr(C)]
#[derive(StableAbi, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ParamCreateItem {
    pub name: RString,
    pub initial_count: u32,
    pub price: Cents,
}

/// This specifies how `Command_NE` is serialized.
///
/// `Command_Interface` was generated by `#[derive(StableAbi)]`,
/// because `trait()` was passed as a parameter to
/// `#[sabi(kind(WithNonExhaustive( ... )))]`.
impl SerializeEnum<Command_NE> for Command_Interface {
    /// The intermediate type the enum is converted into with `SerializeEnum::serialize_enum`,
    /// and then serialized.
    type Proxy = RawValueBox;

    fn serialize_enum(this: &Command_NE) -> Result<RawValueBox, RBoxError> {
        ShopMod_Ref::get_module().unwrap().serialize_command()(this).into_result()
    }
}

/// This specifies how `Command_NE` is deserialized.
impl<'a> DeserializeEnum<'a, Command_NE> for Command_Interface {
    /// The intermediate type that is deserialized,
    /// and then converted to the enum with `DeserializeEnum::deserialize_enum`.
    type Proxy = RawValueRef<'a>;

    fn deserialize_enum(s: RawValueRef<'a>) -> Result<Command_NE, RBoxError> {
        ShopMod_Ref::get_module().unwrap().deserialize_command()(s.get_rstr()).into_result()
    }
}

#[test]
#[cfg(feature = "v1_1")]
fn examples_of_constructing_a_command() {
    use abi_stable::nonexhaustive_enum::NonExhaustive;

    let id = ItemId { id: 0 };

    // Constructing a Command::CreateItem wrapped in NonExhaustive
    // using the constructor generated by using #[sabi(with_boxed_constructor)] on the variant.
    assert_eq!(
        Command::CreateItem_NE(ParamCreateItem {
            name: "foo".into(),
            initial_count: 1,
            price: Cents { cents: 1 },
        }),
        {
            let x = ParamCreateItem {
                name: "foo".into(),
                initial_count: 1,
                price: Cents { cents: 1 },
            };
            let x = RBox::new(x);
            let x = Command::CreateItem(x);
            NonExhaustive::new(x)
        }
    );

    // Constructing a Command::RemoveItem wrapped in NonExhaustive
    // without using the constructors generated using
    // either #[sabi(with_constructor)] or #[sabi(with_boxed_constructor)],
    // since neither attribute was applied to the enum or the variant.
    {
        let x = Command::RemoveItem { id, count: 1 };
        let _ = NonExhaustive::new(x);
    }

    // Constructing a Command::Many wrapped in NonExhaustive
    // using the constructor genereated by using #[sabi(with_constructor)] on the variant
    assert_eq!(Command::Many_NE(RVec::new()), {
        let x = Command::Many { list: RVec::new() };
        NonExhaustive::new(x)
    });
}

///////////////////////////////////////////////////////////////////////////////

/// The return value of `Shop::run_command`.
///
/// Every variant of this enum corresponds to a variant of `Command`.
#[non_exhaustive]
#[repr(u8)]
#[derive(StableAbi, Debug, Clone, PartialEq, Deserialize, Serialize)]
#[sabi(kind(WithNonExhaustive(
    size = [usize;6],
    interface = Command_Interface,
    assert_nonexhaustive = ReturnVal,
)))]
pub enum ReturnVal {
    CreateItem {
        count: u32,
        id: ItemId,
    },
    DeleteItem {
        id: ItemId,
    },
    AddItem {
        remaining: u32,
        id: ItemId,
    },
    RemoveItem {
        removed: u32,
        remaining: u32,
        id: ItemId,
    },
    /// This variant was added in the 1.1 version of the library.
    ///
    /// `#[sabi(with_boxed_constructor)]` tells the `StableAbi` derive macro to
    /// generate the `fn RenameItem_NE(RetRenameItem)->ReturnVal_NE` associated function.
    #[cfg(feature = "v1_1")]
    #[sabi(with_boxed_constructor)]
    RenameItem(RBox<RetRenameItem>),

    /// This variant was added in the 1.1 version of the library.
    ///
    /// `#[sabi(with_constructor)]` tells the `StableAbi` derive macro to
    /// generate the `fn Many_NE(RVec<ReturnVal_NE>)->ReturnVal_NE` associated function.
    #[cfg(feature = "v1_1")]
    #[sabi(with_constructor)]
    Many {
        list: RVec<ReturnVal_NE>,
    },
}

/*
//This was generated by the StableAbi derive macro on ReturnVal.
pub type ReturnVal_NE=
    NonExhaustive<
        ReturnVal,
        ReturnVal_Storage,
        Command_Interface,
    >;
*/

/// A command to rename an item in the shop,
/// which must be wrapped in `ReturnVal::RenameItem_NE` to pass to `Shop::run_command`.
#[cfg(feature = "v1_1")]
#[repr(C)]
#[derive(StableAbi, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RetRenameItem {
    pub id: ItemId,
    pub new_name: RString,
    pub old_name: RString,
}

/// This specifies how `ReturnVal_NE` is serialized.
///
/// This is implemented on Command_Interface,
/// because `interface = Command_Interface` was passed as a parameter to
/// `#[sabi(kind(WithNonExhaustive( ... )))]`.
impl SerializeEnum<ReturnVal_NE> for Command_Interface {
    /// The intermediate type the enum is converted into with `SerializeEnum::serialize_enum`,
    /// and then serialized.
    type Proxy = RawValueBox;

    fn serialize_enum(this: &ReturnVal_NE) -> Result<RawValueBox, RBoxError> {
        ShopMod_Ref::get_module().unwrap().serialize_ret_val()(this).into_result()
    }
}

/// This specifies how `ReturnVal_NE` is deserialized.
impl<'a> DeserializeEnum<'a, ReturnVal_NE> for Command_Interface {
    /// The intermediate type that is deserialized,
    /// and then converted to the enum with `DeserializeEnum::deserialize_enum`.
    type Proxy = RawValueRef<'a>;

    fn deserialize_enum(s: RawValueRef<'a>) -> Result<ReturnVal_NE, RBoxError> {
        ShopMod_Ref::get_module().unwrap().deserialize_ret_val()(s.get_rstr()).into_result()
    }
}

#[test]
#[cfg(feature = "v1_1")]
fn examples_of_constructing_a_returnval() {
    use abi_stable::nonexhaustive_enum::NonExhaustive;

    let id = ItemId { id: 0 };

    // Constructing a ReturnVal::RemoveItem wrapped in NonExhaustive
    // without using the constructors generated using
    // either #[sabi(with_constructor)] or #[sabi(with_boxed_constructor)],
    // since neither attribute was applied to the enum or the variant.
    {
        let x = ReturnVal::RemoveItem {
            removed: 0,
            remaining: 0,
            id,
        };
        let _ = NonExhaustive::new(x);
    }

    // Constructing a ReturnVal::RenameItem wrapped in NonExhaustive
    // using the constructor generated by using #[sabi(with_boxed_constructor)] on the variant
    assert_eq!(
        ReturnVal::RenameItem_NE(RetRenameItem {
            id,
            new_name: "foo".into(),
            old_name: "bar".into(),
        }),
        {
            let x = RetRenameItem {
                id,
                new_name: "foo".into(),
                old_name: "bar".into(),
            };
            let x = RBox::new(x);
            let x = ReturnVal::RenameItem(x);
            NonExhaustive::new(x)
        }
    );

    // Constructing a ReturnVal::Many wrapped in NonExhaustive
    // using the constructor genereated by using #[sabi(with_constructor)] on the variant
    assert_eq!(ReturnVal::Many_NE(RVec::new()), {
        let x = ReturnVal::Many { list: RVec::new() };
        NonExhaustive::new(x)
    });
}

///////////////////////////////////////////////////////////////////////////////

#[non_exhaustive]
#[repr(u8)]
#[derive(StableAbi, Debug, Clone, PartialEq)]
#[sabi(kind(WithNonExhaustive(size = [usize;6], traits(Send, Sync, Debug, Clone, PartialEq),)))]
#[sabi(with_constructor)]
pub enum Error {
    ItemAlreadyExists {
        id: ItemId,
        name: RString,
    },
    ItemIdNotFound {
        id: ItemId,
    },
    #[sabi(with_boxed_constructor)]
    InvalidCommand {
        cmd: RBox<Command_NE>,
    },
}

// Because Error has the `#[sabi(with_constructor)]` attribute applied to it,
// StableAbi generates constructor functions for each variant.
// InvalidCommand overrides it with `#[sabi(with_boxed_constructor)]`,
// which generates constructor functions for variants which wrap a pointer.
#[test]
#[cfg(feature = "v1_1")]
fn examples_of_constructing_an_error() {
    use abi_stable::nonexhaustive_enum::NonExhaustive;

    let id = ItemId { id: 0 };

    assert_eq!(Error::ItemAlreadyExists_NE(id, "hello".into()), {
        let x = Error::ItemAlreadyExists {
            id,
            name: "hello".into(),
        };
        NonExhaustive::new(x)
    });

    assert_eq!(Error::ItemIdNotFound_NE(id), {
        let x = Error::ItemIdNotFound { id };
        NonExhaustive::new(x)
    });

    assert_eq!(Error::InvalidCommand_NE(Command::Many_NE(rvec![])), {
        let x = Command::Many{list: rvec![]};
        let x = NonExhaustive::new(x);
        let x = RBox::new(x);
        let x = Error::InvalidCommand { cmd: x };
        NonExhaustive::new(x)
    });
}

/// The root module of the `shop` dynamic library.
///
/// To load this module,
/// call <ShopMod_Ref as RootModule>::load_from_directory(some_directory_path)
#[repr(C)]
#[derive(StableAbi)]
#[sabi(kind(Prefix(prefix_ref = ShopMod_Ref)))]
#[sabi(missing_field(panic))]
pub struct ShopMod {
    /// Constructs the `Shop_TO` trait object.
    pub new: extern "C" fn() -> Shop_TO<'static, RBox<()>>,

    /// Deserializes a `Command_NE`.
    pub deserialize_command: extern "C" fn(s: RStr<'_>) -> RResult<Command_NE, RBoxError>,

    /// Deserializes a `ReturnVal_NE`.
    pub deserialize_ret_val: extern "C" fn(s: RStr<'_>) -> RResult<ReturnVal_NE, RBoxError>,

    /// Serializes a `Command_NE`.
    pub serialize_command: extern "C" fn(&Command_NE) -> RResult<RawValueBox, RBoxError>,

    /// Serializes a `ReturnVal_NE`.
    ///
    /// The `#[sabi(last_prefix_field)]` attribute here means that this is the last field in this struct
    /// that was defined in the first compatible version of the library
    /// (0.1.0, 0.2.0, 0.3.0, 1.0.0, 2.0.0 ,etc),
    /// requiring new fields to always be added below preexisting ones.
    ///
    /// The `#[sabi(last_prefix_field)]` attribute would stay on this field until the library
    /// bumps its "major" version,
    /// at which point it would be moved to the last field at the time.
    ///
    #[sabi(last_prefix_field)]
    pub serialize_ret_val: extern "C" fn(&ReturnVal_NE) -> RResult<RawValueBox, RBoxError>,
}

impl RootModule for ShopMod_Ref {
    declare_root_module_statics! {ShopMod_Ref}

    const BASE_NAME: &'static str = "shop";
    const NAME: &'static str = "shop";
    const VERSION_STRINGS: VersionStrings = package_version_strings!();
}

/// This represents a shop manager,
/// which can be sent a growing list of commands with each version.
#[sabi_trait]
pub trait Shop {
    /// Runs the `cmd` command.
    ///
    ///
    /// The `#[sabi(last_prefix_field)]` attribute here means that this is the last method
    /// that was defined in the first compatible version of the library
    /// (0.1.0, 0.2.0, 0.3.0, 1.0.0, 2.0.0 ,etc),
    /// requiring new methods to always be added below preexisting ones.
    ///
    /// The `#[sabi(last_prefix_field)]` attribute would stay on this method until the library
    /// bumps its "major" version,
    /// at which point it would be moved to the last method at the time.
    ///
    #[sabi(last_prefix_field)]
    fn run_command(&mut self, cmd: Command_NE) -> RResult<ReturnVal_NE, NonExhaustiveFor<Error>>;
}
