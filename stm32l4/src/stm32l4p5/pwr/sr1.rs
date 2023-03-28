#[doc = "Register `SR1` reader"]
pub struct R(crate::R<SR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `WUF1` reader - Wakeup flag 1"]
pub type WUF1_R = crate::BitReader<WUF1_A>;
#[doc = "Wakeup flag 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUF1_A {
    #[doc = "0: This bit is set when a wakeup event is detected on wakeup pin, WKUPx"]
    Set = 0,
    #[doc = "1: No wakeup event detected on WKUPx"]
    Cleared = 1,
}
impl From<WUF1_A> for bool {
    #[inline(always)]
    fn from(variant: WUF1_A) -> Self {
        variant as u8 != 0
    }
}
impl WUF1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUF1_A {
        match self.bits {
            false => WUF1_A::Set,
            true => WUF1_A::Cleared,
        }
    }
    #[doc = "Checks if the value of the field is `Set`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == WUF1_A::Set
    }
    #[doc = "Checks if the value of the field is `Cleared`"]
    #[inline(always)]
    pub fn is_cleared(&self) -> bool {
        *self == WUF1_A::Cleared
    }
}
#[doc = "Field `WUF2` reader - Wakeup flag 2"]
pub use WUF1_R as WUF2_R;
#[doc = "Field `WUF3` reader - Wakeup flag 3"]
pub use WUF1_R as WUF3_R;
#[doc = "Field `WUF4` reader - Wakeup flag 4"]
pub use WUF1_R as WUF4_R;
#[doc = "Field `WUF5` reader - Wakeup flag 5"]
pub use WUF1_R as WUF5_R;
#[doc = "Field `SBF` reader - Standby flag"]
pub type SBF_R = crate::BitReader<SBF_A>;
#[doc = "Standby flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SBF_A {
    #[doc = "0: The device did not enter the Standby mode"]
    Set = 0,
    #[doc = "1: The device entered the Standby mode"]
    Cleared = 1,
}
impl From<SBF_A> for bool {
    #[inline(always)]
    fn from(variant: SBF_A) -> Self {
        variant as u8 != 0
    }
}
impl SBF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SBF_A {
        match self.bits {
            false => SBF_A::Set,
            true => SBF_A::Cleared,
        }
    }
    #[doc = "Checks if the value of the field is `Set`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == SBF_A::Set
    }
    #[doc = "Checks if the value of the field is `Cleared`"]
    #[inline(always)]
    pub fn is_cleared(&self) -> bool {
        *self == SBF_A::Cleared
    }
}
#[doc = "Field `EXT_SMPS_RDY` reader - External SMPS ready"]
pub type EXT_SMPS_RDY_R = crate::BitReader<EXT_SMPS_RDY_A>;
#[doc = "External SMPS ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EXT_SMPS_RDY_A {
    #[doc = "0: Internal regulator not ready in Range 2, the external SMPS cannot be connected"]
    NotReady = 0,
    #[doc = "1: Internal regulator ready in Range 2, the external SMPS can be connected"]
    Ready = 1,
}
impl From<EXT_SMPS_RDY_A> for bool {
    #[inline(always)]
    fn from(variant: EXT_SMPS_RDY_A) -> Self {
        variant as u8 != 0
    }
}
impl EXT_SMPS_RDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXT_SMPS_RDY_A {
        match self.bits {
            false => EXT_SMPS_RDY_A::NotReady,
            true => EXT_SMPS_RDY_A::Ready,
        }
    }
    #[doc = "Checks if the value of the field is `NotReady`"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == EXT_SMPS_RDY_A::NotReady
    }
    #[doc = "Checks if the value of the field is `Ready`"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == EXT_SMPS_RDY_A::Ready
    }
}
#[doc = "Field `WUFI` reader - Wakeup flag internal"]
pub type WUFI_R = crate::BitReader<WUFI_A>;
#[doc = "Wakeup flag internal\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUFI_A {
    #[doc = "0: This bit is set when a wakeup is detected on the internal wakeup line"]
    Set = 0,
    #[doc = "1: It is cleared when all internal wakeup sources are cleared"]
    Cleared = 1,
}
impl From<WUFI_A> for bool {
    #[inline(always)]
    fn from(variant: WUFI_A) -> Self {
        variant as u8 != 0
    }
}
impl WUFI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUFI_A {
        match self.bits {
            false => WUFI_A::Set,
            true => WUFI_A::Cleared,
        }
    }
    #[doc = "Checks if the value of the field is `Set`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == WUFI_A::Set
    }
    #[doc = "Checks if the value of the field is `Cleared`"]
    #[inline(always)]
    pub fn is_cleared(&self) -> bool {
        *self == WUFI_A::Cleared
    }
}
impl R {
    #[doc = "Bit 0 - Wakeup flag 1"]
    #[inline(always)]
    pub fn wuf1(&self) -> WUF1_R {
        WUF1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Wakeup flag 2"]
    #[inline(always)]
    pub fn wuf2(&self) -> WUF2_R {
        WUF2_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Wakeup flag 3"]
    #[inline(always)]
    pub fn wuf3(&self) -> WUF3_R {
        WUF3_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Wakeup flag 4"]
    #[inline(always)]
    pub fn wuf4(&self) -> WUF4_R {
        WUF4_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Wakeup flag 5"]
    #[inline(always)]
    pub fn wuf5(&self) -> WUF5_R {
        WUF5_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Standby flag"]
    #[inline(always)]
    pub fn sbf(&self) -> SBF_R {
        SBF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 13 - External SMPS ready"]
    #[inline(always)]
    pub fn ext_smps_rdy(&self) -> EXT_SMPS_RDY_R {
        EXT_SMPS_RDY_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - Wakeup flag internal"]
    #[inline(always)]
    pub fn wufi(&self) -> WUFI_R {
        WUFI_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "Power status register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr1](index.html) module"]
pub struct SR1_SPEC;
impl crate::RegisterSpec for SR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sr1::R](R) reader structure"]
impl crate::Readable for SR1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SR1 to value 0"]
impl crate::Resettable for SR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
