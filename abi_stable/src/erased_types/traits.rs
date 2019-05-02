
/*!
Traits for types wrapped in `DynTrait<_>`
*/

use std::{mem,marker::PhantomData};

use crate::{
    StableAbi,
    erased_types::{DynTraitBound},
    std_types::{
        RBoxError, 
        RCow, RStr,StaticStr,
        utypeid::{UTypeId,none_utypeid,some_utypeid},
        ROption,
    },
    version::VersionStrings,
    return_value_equality::ReturnValueEquality,
};

use super::TypeInfo;

#[allow(unused_imports)]
use crate::type_level::bools::{False, True};

/**
An `implementation type`,
with an associated `interface type` which describes the traits that 
must be implemented when constructing a `DynTrait` from Self,
using the `from_value` and `from_ptr` constructors,
so as to pass an opaque type across ffi.

To initialize `INFO` you can use the `impl_get_type_info` macro.

# Uniqueness

Users of this trait can't enforce that they are the only ones with the same interface,
therefore they should handle the `Err(..)`s returned
from the `DynTrait::*_unerased` functions whenever
the convert back and forth between `Self` and `Self::Interface`.


*/
pub trait ImplType: Sized  {
    type Interface: InterfaceType;

    const INFO: &'static TypeInfo;
}

/**
Defines the usable/required traits when creating a 
`DynTrait<Pointer<ZeroSized< ThisType >>>`
from a type that implements `ImplType<Interface= ThisType >` .

This trait can only be implemented within the `impl_InterfaceType` macro,
giving a default value to each associated type,
so that adding associated types is not a breaking change.

The value of every associated type is `True`/`False`.

On `True`,the trait would be required by and usable in `DynTrait`.

On `False`,the trait would not be required by and not usable in `DynTrait`.

# Example

```

use abi_stable::{
    StableAbi,
    impl_InterfaceType,
    erased_types::InterfaceType,
    type_level::bools::*,
};

#[repr(C)]
#[derive(StableAbi)]
pub struct FooInterface;

impl_InterfaceType!{
    impl InterfaceType for FooInterface {
        type Clone=True;

        type Debug=True;

        /////////////////////////////////////    
        //// defaulted associated types
        /////////////////////////////////////

        // type Default=False;

        // type Display=False;

        // type Serialize=False;

        // type Eq=False;

        // type PartialEq=False;

        // type Ord=False;

        // type PartialOrd=False;

        // type Hash=False;

        // type Deserialize=False;
    }
}

# fn main(){}


```


*/
pub trait InterfaceType: Sized + 'static + StableAbi {
    type Send;

    type Sync;

    type Clone;

    type Default;

    type Display;

    type Debug;

    type Serialize;

    type Eq;

    type PartialEq;

    type Ord;

    type PartialOrd;

    type Hash;

    type Deserialize;

    #[doc(hidden)]
    type define_this_in_the_impl_InterfaceType_macro;

    // type FmtWrite;
    // type IoWrite;
    // type IoRead;
    // type IoBufRead;
}



/**
Describes how this `implementation type` is serialized.
*/
pub trait SerializeImplType {
    fn serialize_impl<'a>(&'a self) -> Result<RCow<'a, RStr<'a>>, RBoxError>;
}

/**
Describes how this `interface type` is deserialized.

Generally this delegates to a library function,so that the implementation can be delegated
to the `implementation crate`.

*/
pub trait DeserializeOwnedInterface<'borr>: InterfaceType<Deserialize = True> {
    type Deserialized: DynTraitBound<'borr,Interface = Self>+'borr;

    fn deserialize_impl(s: RStr<'_>) -> Result<Self::Deserialized, RBoxError>;
}


pub trait DeserializeBorrowedInterface<'borr>: InterfaceType<Deserialize = True> {
    type Deserialized: DynTraitBound<'borr,Interface = Self>+'borr;

    fn deserialize_impl(s: RStr<'borr>) -> Result<Self::Deserialized, RBoxError> ;
}



/////////////////////////////////////////////////////////////////////





//////////////////////////////////////////////////////////////////


/// Helper struct for Wrapping any type in a 
/// `DynTrait<Pointer< OpaqueTyoe< Interface > >>`.
pub struct InterfaceFor<T,Interface,IsStatic>(
    PhantomData<fn()->(T,Interface,IsStatic)>
);

impl<T,Interface,IsStatic> ImplType for InterfaceFor<T,Interface,IsStatic>
where 
    Interface:InterfaceType,
    T:GetUTID<IsStatic>,
{
    type Interface=Interface;
    
    const INFO:&'static TypeInfo=&TypeInfo{
        size:mem::size_of::<T>(),
        alignment:mem::align_of::<T>(),
        _uid:<T as GetUTID<IsStatic>>::UID,
        name:StaticStr::new("<erased>"),
        file:StaticStr::new("<unavailable>"),
        package:StaticStr::new("<unavailable>"),
        package_version:VersionStrings{
            major:StaticStr::new("99"),
            minor:StaticStr::new("99"),
            patch:StaticStr::new("99"),
        },
        _private_field:(),
    };
}


#[doc(hidden)]
pub trait GetUTID<IsStatic>{
    const UID:ReturnValueEquality<ROption<UTypeId>>;
}


impl<T> GetUTID<True> for T
where T:'static
{
    const UID:ReturnValueEquality<ROption<UTypeId>>=ReturnValueEquality{
        function:some_utypeid::<T>
    };
}

impl<T> GetUTID<False> for T{
    const UID:ReturnValueEquality<ROption<UTypeId>>=ReturnValueEquality{
        function:none_utypeid
    };
}



/////////////////////////////////////////////////////////////////////

crate::impl_InterfaceType!{
    impl crate::erased_types::InterfaceType for () {
        type Send=True;
        type Sync=True;
    }
}
