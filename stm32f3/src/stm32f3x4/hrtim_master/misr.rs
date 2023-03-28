#[doc = "Register `MISR` reader"]
pub struct R(crate::R<MISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MCMP1` reader - Master Compare 1 Interrupt Flag"]
pub type MCMP1_R = crate::BitReader<MCMP1_A>;
#[doc = "Master Compare 1 Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MCMP1_A {
    #[doc = "0: No master compare interrupt occurred"]
    NoEvent = 0,
    #[doc = "1: Master compare interrupt occurred"]
    Event = 1,
}
impl From<MCMP1_A> for bool {
    #[inline(always)]
    fn from(variant: MCMP1_A) -> Self {
        variant as u8 != 0
    }
}
impl MCMP1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MCMP1_A {
        match self.bits {
            false => MCMP1_A::NoEvent,
            true => MCMP1_A::Event,
        }
    }
    #[doc = "Checks if the value of the field is `NoEvent`"]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == MCMP1_A::NoEvent
    }
    #[doc = "Checks if the value of the field is `Event`"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == MCMP1_A::Event
    }
}
#[doc = "Field `MCMP2` reader - Master Compare 2 Interrupt Flag"]
pub use MCMP1_R as MCMP2_R;
#[doc = "Field `MCMP3` reader - Master Compare 3 Interrupt Flag"]
pub use MCMP1_R as MCMP3_R;
#[doc = "Field `MCMP4` reader - Master Compare 4 Interrupt Flag"]
pub use MCMP1_R as MCMP4_R;
#[doc = "Field `MREP` reader - Master Repetition Interrupt Flag"]
pub type MREP_R = crate::BitReader<MREP_A>;
#[doc = "Master Repetition Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MREP_A {
    #[doc = "0: No master repetition interrupt occurred"]
    NoEvent = 0,
    #[doc = "1: Master repetition interrupt occurred"]
    Event = 1,
}
impl From<MREP_A> for bool {
    #[inline(always)]
    fn from(variant: MREP_A) -> Self {
        variant as u8 != 0
    }
}
impl MREP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MREP_A {
        match self.bits {
            false => MREP_A::NoEvent,
            true => MREP_A::Event,
        }
    }
    #[doc = "Checks if the value of the field is `NoEvent`"]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == MREP_A::NoEvent
    }
    #[doc = "Checks if the value of the field is `Event`"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == MREP_A::Event
    }
}
#[doc = "Field `SYNC` reader - Sync Input Interrupt Flag"]
pub type SYNC_R = crate::BitReader<SYNC_A>;
#[doc = "Sync Input Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYNC_A {
    #[doc = "0: No sync input interrupt occurred"]
    NoEvent = 0,
    #[doc = "1: Sync input interrupt occurred"]
    Event = 1,
}
impl From<SYNC_A> for bool {
    #[inline(always)]
    fn from(variant: SYNC_A) -> Self {
        variant as u8 != 0
    }
}
impl SYNC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYNC_A {
        match self.bits {
            false => SYNC_A::NoEvent,
            true => SYNC_A::Event,
        }
    }
    #[doc = "Checks if the value of the field is `NoEvent`"]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == SYNC_A::NoEvent
    }
    #[doc = "Checks if the value of the field is `Event`"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == SYNC_A::Event
    }
}
#[doc = "Field `MUPD` reader - Master Update Interrupt Flag"]
pub type MUPD_R = crate::BitReader<MUPD_A>;
#[doc = "Master Update Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MUPD_A {
    #[doc = "0: No master update interrupt occurred"]
    NoEvent = 0,
    #[doc = "1: Master update interrupt occurred"]
    Event = 1,
}
impl From<MUPD_A> for bool {
    #[inline(always)]
    fn from(variant: MUPD_A) -> Self {
        variant as u8 != 0
    }
}
impl MUPD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MUPD_A {
        match self.bits {
            false => MUPD_A::NoEvent,
            true => MUPD_A::Event,
        }
    }
    #[doc = "Checks if the value of the field is `NoEvent`"]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == MUPD_A::NoEvent
    }
    #[doc = "Checks if the value of the field is `Event`"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == MUPD_A::Event
    }
}
impl R {
    #[doc = "Bit 0 - Master Compare 1 Interrupt Flag"]
    #[inline(always)]
    pub fn mcmp1(&self) -> MCMP1_R {
        MCMP1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Master Compare 2 Interrupt Flag"]
    #[inline(always)]
    pub fn mcmp2(&self) -> MCMP2_R {
        MCMP2_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Master Compare 3 Interrupt Flag"]
    #[inline(always)]
    pub fn mcmp3(&self) -> MCMP3_R {
        MCMP3_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Master Compare 4 Interrupt Flag"]
    #[inline(always)]
    pub fn mcmp4(&self) -> MCMP4_R {
        MCMP4_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Master Repetition Interrupt Flag"]
    #[inline(always)]
    pub fn mrep(&self) -> MREP_R {
        MREP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Sync Input Interrupt Flag"]
    #[inline(always)]
    pub fn sync(&self) -> SYNC_R {
        SYNC_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Master Update Interrupt Flag"]
    #[inline(always)]
    pub fn mupd(&self) -> MUPD_R {
        MUPD_R::new(((self.bits >> 6) & 1) != 0)
    }
}
#[doc = "Master Timer Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [misr](index.html) module"]
pub struct MISR_SPEC;
impl crate::RegisterSpec for MISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [misr::R](R) reader structure"]
impl crate::Readable for MISR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MISR to value 0"]
impl crate::Resettable for MISR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
