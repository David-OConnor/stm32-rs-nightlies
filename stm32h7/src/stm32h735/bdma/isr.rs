#[doc = "Register `ISR` reader"]
pub struct R(crate::R<ISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `GIF0` reader - Global interrupt flag for channel x"]
pub type GIF0_R = crate::BitReader<GIF0_A>;
#[doc = "Global interrupt flag for channel x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GIF0_A {
    #[doc = "0: No TE, HT or TC event on channel x"]
    NoEvent = 0,
    #[doc = "1: A TE, HT or TC event occurred on channel x"]
    Event = 1,
}
impl From<GIF0_A> for bool {
    #[inline(always)]
    fn from(variant: GIF0_A) -> Self {
        variant as u8 != 0
    }
}
impl GIF0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GIF0_A {
        match self.bits {
            false => GIF0_A::NoEvent,
            true => GIF0_A::Event,
        }
    }
    #[doc = "Checks if the value of the field is `NoEvent`"]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == GIF0_A::NoEvent
    }
    #[doc = "Checks if the value of the field is `Event`"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == GIF0_A::Event
    }
}
#[doc = "Field `TCIF0` reader - Transfer complete (TC) flag for channel x"]
pub type TCIF0_R = crate::BitReader<TCIF0_A>;
#[doc = "Transfer complete (TC) flag for channel x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCIF0_A {
    #[doc = "0: No transfer complete event on channel x"]
    NotComplete = 0,
    #[doc = "1: A transfer complete event occurred on channel x"]
    Complete = 1,
}
impl From<TCIF0_A> for bool {
    #[inline(always)]
    fn from(variant: TCIF0_A) -> Self {
        variant as u8 != 0
    }
}
impl TCIF0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCIF0_A {
        match self.bits {
            false => TCIF0_A::NotComplete,
            true => TCIF0_A::Complete,
        }
    }
    #[doc = "Checks if the value of the field is `NotComplete`"]
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == TCIF0_A::NotComplete
    }
    #[doc = "Checks if the value of the field is `Complete`"]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == TCIF0_A::Complete
    }
}
#[doc = "Field `HTIF0` reader - Half transfer (HT) flag for channel x"]
pub type HTIF0_R = crate::BitReader<HTIF0_A>;
#[doc = "Half transfer (HT) flag for channel x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HTIF0_A {
    #[doc = "0: No half transfer event on channel x"]
    NotHalf = 0,
    #[doc = "1: A half transfer event occurred on channel x"]
    Half = 1,
}
impl From<HTIF0_A> for bool {
    #[inline(always)]
    fn from(variant: HTIF0_A) -> Self {
        variant as u8 != 0
    }
}
impl HTIF0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HTIF0_A {
        match self.bits {
            false => HTIF0_A::NotHalf,
            true => HTIF0_A::Half,
        }
    }
    #[doc = "Checks if the value of the field is `NotHalf`"]
    #[inline(always)]
    pub fn is_not_half(&self) -> bool {
        *self == HTIF0_A::NotHalf
    }
    #[doc = "Checks if the value of the field is `Half`"]
    #[inline(always)]
    pub fn is_half(&self) -> bool {
        *self == HTIF0_A::Half
    }
}
#[doc = "Field `TEIF0` reader - Transfer error (TE) flag for channel x"]
pub type TEIF0_R = crate::BitReader<TEIF0_A>;
#[doc = "Transfer error (TE) flag for channel x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEIF0_A {
    #[doc = "0: No transfer error on channel x"]
    NoError = 0,
    #[doc = "1: A transfer error occurred on channel x"]
    Error = 1,
}
impl From<TEIF0_A> for bool {
    #[inline(always)]
    fn from(variant: TEIF0_A) -> Self {
        variant as u8 != 0
    }
}
impl TEIF0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TEIF0_A {
        match self.bits {
            false => TEIF0_A::NoError,
            true => TEIF0_A::Error,
        }
    }
    #[doc = "Checks if the value of the field is `NoError`"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == TEIF0_A::NoError
    }
    #[doc = "Checks if the value of the field is `Error`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == TEIF0_A::Error
    }
}
#[doc = "Field `GIF1` reader - Global interrupt flag for channel x"]
pub use GIF0_R as GIF1_R;
#[doc = "Field `GIF2` reader - Global interrupt flag for channel x"]
pub use GIF0_R as GIF2_R;
#[doc = "Field `GIF3` reader - Global interrupt flag for channel x"]
pub use GIF0_R as GIF3_R;
#[doc = "Field `GIF4` reader - Global interrupt flag for channel x"]
pub use GIF0_R as GIF4_R;
#[doc = "Field `GIF5` reader - Global interrupt flag for channel x"]
pub use GIF0_R as GIF5_R;
#[doc = "Field `GIF6` reader - Global interrupt flag for channel x"]
pub use GIF0_R as GIF6_R;
#[doc = "Field `GIF7` reader - Global interrupt flag for channel x"]
pub use GIF0_R as GIF7_R;
#[doc = "Field `HTIF1` reader - Half transfer (HT) flag for channel x"]
pub use HTIF0_R as HTIF1_R;
#[doc = "Field `HTIF2` reader - Half transfer (HT) flag for channel x"]
pub use HTIF0_R as HTIF2_R;
#[doc = "Field `HTIF3` reader - Half transfer (HT) flag for channel x"]
pub use HTIF0_R as HTIF3_R;
#[doc = "Field `HTIF4` reader - Half transfer (HT) flag for channel x"]
pub use HTIF0_R as HTIF4_R;
#[doc = "Field `HTIF5` reader - Half transfer (HT) flag for channel x"]
pub use HTIF0_R as HTIF5_R;
#[doc = "Field `HTIF6` reader - Half transfer (HT) flag for channel x"]
pub use HTIF0_R as HTIF6_R;
#[doc = "Field `HTIF7` reader - Half transfer (HT) flag for channel x"]
pub use HTIF0_R as HTIF7_R;
#[doc = "Field `TCIF1` reader - Transfer complete (TC) flag for channel x"]
pub use TCIF0_R as TCIF1_R;
#[doc = "Field `TCIF2` reader - Transfer complete (TC) flag for channel x"]
pub use TCIF0_R as TCIF2_R;
#[doc = "Field `TCIF3` reader - Transfer complete (TC) flag for channel x"]
pub use TCIF0_R as TCIF3_R;
#[doc = "Field `TCIF4` reader - Transfer complete (TC) flag for channel x"]
pub use TCIF0_R as TCIF4_R;
#[doc = "Field `TCIF5` reader - Transfer complete (TC) flag for channel x"]
pub use TCIF0_R as TCIF5_R;
#[doc = "Field `TCIF6` reader - Transfer complete (TC) flag for channel x"]
pub use TCIF0_R as TCIF6_R;
#[doc = "Field `TCIF7` reader - Transfer complete (TC) flag for channel x"]
pub use TCIF0_R as TCIF7_R;
#[doc = "Field `TEIF1` reader - Transfer error (TE) flag for channel x"]
pub use TEIF0_R as TEIF1_R;
#[doc = "Field `TEIF2` reader - Transfer error (TE) flag for channel x"]
pub use TEIF0_R as TEIF2_R;
#[doc = "Field `TEIF3` reader - Transfer error (TE) flag for channel x"]
pub use TEIF0_R as TEIF3_R;
#[doc = "Field `TEIF4` reader - Transfer error (TE) flag for channel x"]
pub use TEIF0_R as TEIF4_R;
#[doc = "Field `TEIF5` reader - Transfer error (TE) flag for channel x"]
pub use TEIF0_R as TEIF5_R;
#[doc = "Field `TEIF6` reader - Transfer error (TE) flag for channel x"]
pub use TEIF0_R as TEIF6_R;
#[doc = "Field `TEIF7` reader - Transfer error (TE) flag for channel x"]
pub use TEIF0_R as TEIF7_R;
impl R {
    #[doc = "Bit 0 - Global interrupt flag for channel x"]
    #[inline(always)]
    pub fn gif0(&self) -> GIF0_R {
        GIF0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transfer complete (TC) flag for channel x"]
    #[inline(always)]
    pub fn tcif0(&self) -> TCIF0_R {
        TCIF0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Half transfer (HT) flag for channel x"]
    #[inline(always)]
    pub fn htif0(&self) -> HTIF0_R {
        HTIF0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transfer error (TE) flag for channel x"]
    #[inline(always)]
    pub fn teif0(&self) -> TEIF0_R {
        TEIF0_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Global interrupt flag for channel x"]
    #[inline(always)]
    pub fn gif1(&self) -> GIF1_R {
        GIF1_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Transfer complete (TC) flag for channel x"]
    #[inline(always)]
    pub fn tcif1(&self) -> TCIF1_R {
        TCIF1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Half transfer (HT) flag for channel x"]
    #[inline(always)]
    pub fn htif1(&self) -> HTIF1_R {
        HTIF1_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transfer error (TE) flag for channel x"]
    #[inline(always)]
    pub fn teif1(&self) -> TEIF1_R {
        TEIF1_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Global interrupt flag for channel x"]
    #[inline(always)]
    pub fn gif2(&self) -> GIF2_R {
        GIF2_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Transfer complete (TC) flag for channel x"]
    #[inline(always)]
    pub fn tcif2(&self) -> TCIF2_R {
        TCIF2_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Half transfer (HT) flag for channel x"]
    #[inline(always)]
    pub fn htif2(&self) -> HTIF2_R {
        HTIF2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Transfer error (TE) flag for channel x"]
    #[inline(always)]
    pub fn teif2(&self) -> TEIF2_R {
        TEIF2_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Global interrupt flag for channel x"]
    #[inline(always)]
    pub fn gif3(&self) -> GIF3_R {
        GIF3_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Transfer complete (TC) flag for channel x"]
    #[inline(always)]
    pub fn tcif3(&self) -> TCIF3_R {
        TCIF3_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Half transfer (HT) flag for channel x"]
    #[inline(always)]
    pub fn htif3(&self) -> HTIF3_R {
        HTIF3_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Transfer error (TE) flag for channel x"]
    #[inline(always)]
    pub fn teif3(&self) -> TEIF3_R {
        TEIF3_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Global interrupt flag for channel x"]
    #[inline(always)]
    pub fn gif4(&self) -> GIF4_R {
        GIF4_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Transfer complete (TC) flag for channel x"]
    #[inline(always)]
    pub fn tcif4(&self) -> TCIF4_R {
        TCIF4_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Half transfer (HT) flag for channel x"]
    #[inline(always)]
    pub fn htif4(&self) -> HTIF4_R {
        HTIF4_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Transfer error (TE) flag for channel x"]
    #[inline(always)]
    pub fn teif4(&self) -> TEIF4_R {
        TEIF4_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Global interrupt flag for channel x"]
    #[inline(always)]
    pub fn gif5(&self) -> GIF5_R {
        GIF5_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Transfer complete (TC) flag for channel x"]
    #[inline(always)]
    pub fn tcif5(&self) -> TCIF5_R {
        TCIF5_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Half transfer (HT) flag for channel x"]
    #[inline(always)]
    pub fn htif5(&self) -> HTIF5_R {
        HTIF5_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Transfer error (TE) flag for channel x"]
    #[inline(always)]
    pub fn teif5(&self) -> TEIF5_R {
        TEIF5_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Global interrupt flag for channel x"]
    #[inline(always)]
    pub fn gif6(&self) -> GIF6_R {
        GIF6_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Transfer complete (TC) flag for channel x"]
    #[inline(always)]
    pub fn tcif6(&self) -> TCIF6_R {
        TCIF6_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Half transfer (HT) flag for channel x"]
    #[inline(always)]
    pub fn htif6(&self) -> HTIF6_R {
        HTIF6_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Transfer error (TE) flag for channel x"]
    #[inline(always)]
    pub fn teif6(&self) -> TEIF6_R {
        TEIF6_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Global interrupt flag for channel x"]
    #[inline(always)]
    pub fn gif7(&self) -> GIF7_R {
        GIF7_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Transfer complete (TC) flag for channel x"]
    #[inline(always)]
    pub fn tcif7(&self) -> TCIF7_R {
        TCIF7_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Half transfer (HT) flag for channel x"]
    #[inline(always)]
    pub fn htif7(&self) -> HTIF7_R {
        HTIF7_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Transfer error (TE) flag for channel x"]
    #[inline(always)]
    pub fn teif7(&self) -> TEIF7_R {
        TEIF7_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isr](index.html) module"]
pub struct ISR_SPEC;
impl crate::RegisterSpec for ISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [isr::R](R) reader structure"]
impl crate::Readable for ISR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ISR to value 0"]
impl crate::Resettable for ISR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
