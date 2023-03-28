#[doc = "Register `SR` reader"]
pub struct R(crate::R<SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TAMP1F` reader - TAMP1F"]
pub type TAMP1F_R = crate::BitReader<TAMP1F_A>;
#[doc = "TAMP1F\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TAMP1F_A {
    #[doc = "0: No tamper detected"]
    Idle = 0,
    #[doc = "1: Tamper detected"]
    Tamper = 1,
}
impl From<TAMP1F_A> for bool {
    #[inline(always)]
    fn from(variant: TAMP1F_A) -> Self {
        variant as u8 != 0
    }
}
impl TAMP1F_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TAMP1F_A {
        match self.bits {
            false => TAMP1F_A::Idle,
            true => TAMP1F_A::Tamper,
        }
    }
    #[doc = "Checks if the value of the field is `Idle`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == TAMP1F_A::Idle
    }
    #[doc = "Checks if the value of the field is `Tamper`"]
    #[inline(always)]
    pub fn is_tamper(&self) -> bool {
        *self == TAMP1F_A::Tamper
    }
}
#[doc = "Field `TAMP2F` reader - TAMP2F"]
pub use TAMP1F_R as TAMP2F_R;
#[doc = "Field `TAMP3F` reader - TAMP3F"]
pub use TAMP1F_R as TAMP3F_R;
#[doc = "Field `ITAMP3F` reader - ITAMP3F"]
pub type ITAMP3F_R = crate::BitReader<ITAMP3F_A>;
#[doc = "ITAMP3F\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ITAMP3F_A {
    #[doc = "0: No tamper detected"]
    Idle = 0,
    #[doc = "1: Internal tamper detected"]
    Tamper = 1,
}
impl From<ITAMP3F_A> for bool {
    #[inline(always)]
    fn from(variant: ITAMP3F_A) -> Self {
        variant as u8 != 0
    }
}
impl ITAMP3F_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ITAMP3F_A {
        match self.bits {
            false => ITAMP3F_A::Idle,
            true => ITAMP3F_A::Tamper,
        }
    }
    #[doc = "Checks if the value of the field is `Idle`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == ITAMP3F_A::Idle
    }
    #[doc = "Checks if the value of the field is `Tamper`"]
    #[inline(always)]
    pub fn is_tamper(&self) -> bool {
        *self == ITAMP3F_A::Tamper
    }
}
#[doc = "Field `ITAMP5F` reader - ITAMP5F"]
pub use ITAMP3F_R as ITAMP5F_R;
#[doc = "Field `ITAMP6F` reader - ITAMP6F"]
pub use ITAMP3F_R as ITAMP6F_R;
#[doc = "Field `ITAMP8F` reader - ITAMP8F"]
pub use ITAMP3F_R as ITAMP8F_R;
impl R {
    #[doc = "Bit 0 - TAMP1F"]
    #[inline(always)]
    pub fn tamp1f(&self) -> TAMP1F_R {
        TAMP1F_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TAMP2F"]
    #[inline(always)]
    pub fn tamp2f(&self) -> TAMP2F_R {
        TAMP2F_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TAMP3F"]
    #[inline(always)]
    pub fn tamp3f(&self) -> TAMP3F_R {
        TAMP3F_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 18 - ITAMP3F"]
    #[inline(always)]
    pub fn itamp3f(&self) -> ITAMP3F_R {
        ITAMP3F_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - ITAMP5F"]
    #[inline(always)]
    pub fn itamp5f(&self) -> ITAMP5F_R {
        ITAMP5F_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - ITAMP6F"]
    #[inline(always)]
    pub fn itamp6f(&self) -> ITAMP6F_R {
        ITAMP6F_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 23 - ITAMP8F"]
    #[inline(always)]
    pub fn itamp8f(&self) -> ITAMP8F_R {
        ITAMP8F_R::new(((self.bits >> 23) & 1) != 0)
    }
}
#[doc = "TAMP status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](index.html) module"]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sr::R](R) reader structure"]
impl crate::Readable for SR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
