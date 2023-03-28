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
#[doc = "Field `CCF` reader - Computation complete flag"]
pub type CCF_R = crate::BitReader<CCF_A>;
#[doc = "Computation complete flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCF_A {
    #[doc = "0: Computation complete"]
    Complete = 0,
    #[doc = "1: Computation not complete"]
    NotComplete = 1,
}
impl From<CCF_A> for bool {
    #[inline(always)]
    fn from(variant: CCF_A) -> Self {
        variant as u8 != 0
    }
}
impl CCF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCF_A {
        match self.bits {
            false => CCF_A::Complete,
            true => CCF_A::NotComplete,
        }
    }
    #[doc = "Checks if the value of the field is `Complete`"]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == CCF_A::Complete
    }
    #[doc = "Checks if the value of the field is `NotComplete`"]
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == CCF_A::NotComplete
    }
}
#[doc = "Field `RDERR` reader - Read error flag"]
pub type RDERR_R = crate::BitReader<RDERR_A>;
#[doc = "Read error flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RDERR_A {
    #[doc = "0: Read error not detected"]
    NoError = 0,
    #[doc = "1: Read error detected"]
    Error = 1,
}
impl From<RDERR_A> for bool {
    #[inline(always)]
    fn from(variant: RDERR_A) -> Self {
        variant as u8 != 0
    }
}
impl RDERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RDERR_A {
        match self.bits {
            false => RDERR_A::NoError,
            true => RDERR_A::Error,
        }
    }
    #[doc = "Checks if the value of the field is `NoError`"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == RDERR_A::NoError
    }
    #[doc = "Checks if the value of the field is `Error`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == RDERR_A::Error
    }
}
#[doc = "Field `WRERR` reader - Write error flag"]
pub type WRERR_R = crate::BitReader<WRERR_A>;
#[doc = "Write error flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WRERR_A {
    #[doc = "0: Write error not detected"]
    NoError = 0,
    #[doc = "1: Write error detected"]
    Error = 1,
}
impl From<WRERR_A> for bool {
    #[inline(always)]
    fn from(variant: WRERR_A) -> Self {
        variant as u8 != 0
    }
}
impl WRERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WRERR_A {
        match self.bits {
            false => WRERR_A::NoError,
            true => WRERR_A::Error,
        }
    }
    #[doc = "Checks if the value of the field is `NoError`"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == WRERR_A::NoError
    }
    #[doc = "Checks if the value of the field is `Error`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == WRERR_A::Error
    }
}
#[doc = "Field `BUSY` reader - Busy"]
pub type BUSY_R = crate::BitReader<BUSY_A>;
#[doc = "Busy\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUSY_A {
    #[doc = "0: Idle"]
    Idle = 0,
    #[doc = "1: Busy"]
    Busy = 1,
}
impl From<BUSY_A> for bool {
    #[inline(always)]
    fn from(variant: BUSY_A) -> Self {
        variant as u8 != 0
    }
}
impl BUSY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUSY_A {
        match self.bits {
            false => BUSY_A::Idle,
            true => BUSY_A::Busy,
        }
    }
    #[doc = "Checks if the value of the field is `Idle`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == BUSY_A::Idle
    }
    #[doc = "Checks if the value of the field is `Busy`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == BUSY_A::Busy
    }
}
impl R {
    #[doc = "Bit 0 - Computation complete flag"]
    #[inline(always)]
    pub fn ccf(&self) -> CCF_R {
        CCF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Read error flag"]
    #[inline(always)]
    pub fn rderr(&self) -> RDERR_R {
        RDERR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write error flag"]
    #[inline(always)]
    pub fn wrerr(&self) -> WRERR_R {
        WRERR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Busy"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](index.html) module"]
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
