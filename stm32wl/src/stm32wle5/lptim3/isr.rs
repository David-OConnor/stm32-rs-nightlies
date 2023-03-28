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
#[doc = "Field `CMPM` reader - Compare match"]
pub type CMPM_R = crate::BitReader<CMPMR_A>;
#[doc = "Compare match\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPMR_A {
    #[doc = "1: Compare match"]
    Set = 1,
}
impl From<CMPMR_A> for bool {
    #[inline(always)]
    fn from(variant: CMPMR_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CMPMR_A> {
        match self.bits {
            true => Some(CMPMR_A::Set),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Set`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == CMPMR_A::Set
    }
}
#[doc = "Field `ARRM` reader - Autoreload match"]
pub type ARRM_R = crate::BitReader<ARRMR_A>;
#[doc = "Autoreload match\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ARRMR_A {
    #[doc = "1: Autoreload match"]
    Set = 1,
}
impl From<ARRMR_A> for bool {
    #[inline(always)]
    fn from(variant: ARRMR_A) -> Self {
        variant as u8 != 0
    }
}
impl ARRM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ARRMR_A> {
        match self.bits {
            true => Some(ARRMR_A::Set),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Set`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == ARRMR_A::Set
    }
}
#[doc = "Field `EXTTRIG` reader - External trigger edge event"]
pub type EXTTRIG_R = crate::BitReader<EXTTRIGR_A>;
#[doc = "External trigger edge event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EXTTRIGR_A {
    #[doc = "1: External trigger edge event"]
    Set = 1,
}
impl From<EXTTRIGR_A> for bool {
    #[inline(always)]
    fn from(variant: EXTTRIGR_A) -> Self {
        variant as u8 != 0
    }
}
impl EXTTRIG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EXTTRIGR_A> {
        match self.bits {
            true => Some(EXTTRIGR_A::Set),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Set`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == EXTTRIGR_A::Set
    }
}
#[doc = "Field `CMPOK` reader - Compare register update OK"]
pub type CMPOK_R = crate::BitReader<CMPOKR_A>;
#[doc = "Compare register update OK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPOKR_A {
    #[doc = "1: Compare register update OK"]
    Set = 1,
}
impl From<CMPOKR_A> for bool {
    #[inline(always)]
    fn from(variant: CMPOKR_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPOK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CMPOKR_A> {
        match self.bits {
            true => Some(CMPOKR_A::Set),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Set`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == CMPOKR_A::Set
    }
}
#[doc = "Field `ARROK` reader - Autoreload register update OK"]
pub type ARROK_R = crate::BitReader<ARROKR_A>;
#[doc = "Autoreload register update OK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ARROKR_A {
    #[doc = "1: Autoreload register update OK"]
    Set = 1,
}
impl From<ARROKR_A> for bool {
    #[inline(always)]
    fn from(variant: ARROKR_A) -> Self {
        variant as u8 != 0
    }
}
impl ARROK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ARROKR_A> {
        match self.bits {
            true => Some(ARROKR_A::Set),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Set`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == ARROKR_A::Set
    }
}
#[doc = "Field `UP` reader - Counter direction change down to up"]
pub type UP_R = crate::BitReader<UPR_A>;
#[doc = "Counter direction change down to up\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UPR_A {
    #[doc = "1: Counter direction change down to up"]
    Set = 1,
}
impl From<UPR_A> for bool {
    #[inline(always)]
    fn from(variant: UPR_A) -> Self {
        variant as u8 != 0
    }
}
impl UP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<UPR_A> {
        match self.bits {
            true => Some(UPR_A::Set),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Set`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == UPR_A::Set
    }
}
#[doc = "Field `DOWN` reader - Counter direction change up to down"]
pub type DOWN_R = crate::BitReader<DOWNR_A>;
#[doc = "Counter direction change up to down\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DOWNR_A {
    #[doc = "1: Counter direction change up to down"]
    Set = 1,
}
impl From<DOWNR_A> for bool {
    #[inline(always)]
    fn from(variant: DOWNR_A) -> Self {
        variant as u8 != 0
    }
}
impl DOWN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DOWNR_A> {
        match self.bits {
            true => Some(DOWNR_A::Set),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Set`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == DOWNR_A::Set
    }
}
#[doc = "Field `UE` reader - LPTIM update event occurred"]
pub type UE_R = crate::BitReader<UER_A>;
#[doc = "LPTIM update event occurred\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UER_A {
    #[doc = "1: LPTIM update event occurred"]
    Set = 1,
}
impl From<UER_A> for bool {
    #[inline(always)]
    fn from(variant: UER_A) -> Self {
        variant as u8 != 0
    }
}
impl UE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<UER_A> {
        match self.bits {
            true => Some(UER_A::Set),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Set`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == UER_A::Set
    }
}
#[doc = "Field `REPOK` reader - Repetition register update Ok"]
pub type REPOK_R = crate::BitReader<REPOKR_A>;
#[doc = "Repetition register update Ok\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REPOKR_A {
    #[doc = "1: Repetition register update OK"]
    Set = 1,
}
impl From<REPOKR_A> for bool {
    #[inline(always)]
    fn from(variant: REPOKR_A) -> Self {
        variant as u8 != 0
    }
}
impl REPOK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<REPOKR_A> {
        match self.bits {
            true => Some(REPOKR_A::Set),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Set`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == REPOKR_A::Set
    }
}
impl R {
    #[doc = "Bit 0 - Compare match"]
    #[inline(always)]
    pub fn cmpm(&self) -> CMPM_R {
        CMPM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Autoreload match"]
    #[inline(always)]
    pub fn arrm(&self) -> ARRM_R {
        ARRM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - External trigger edge event"]
    #[inline(always)]
    pub fn exttrig(&self) -> EXTTRIG_R {
        EXTTRIG_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Compare register update OK"]
    #[inline(always)]
    pub fn cmpok(&self) -> CMPOK_R {
        CMPOK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Autoreload register update OK"]
    #[inline(always)]
    pub fn arrok(&self) -> ARROK_R {
        ARROK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Counter direction change down to up"]
    #[inline(always)]
    pub fn up(&self) -> UP_R {
        UP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Counter direction change up to down"]
    #[inline(always)]
    pub fn down(&self) -> DOWN_R {
        DOWN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - LPTIM update event occurred"]
    #[inline(always)]
    pub fn ue(&self) -> UE_R {
        UE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Repetition register update Ok"]
    #[inline(always)]
    pub fn repok(&self) -> REPOK_R {
        REPOK_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[doc = "interrupt and status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isr](index.html) module"]
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
