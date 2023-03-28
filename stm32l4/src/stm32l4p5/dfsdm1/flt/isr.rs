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
#[doc = "Field `JEOCF` reader - End of injected conversion flag"]
pub type JEOCF_R = crate::BitReader<JEOCF_A>;
#[doc = "End of injected conversion flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JEOCF_A {
    #[doc = "0: No injected conversion has completed"]
    Clear = 0,
    #[doc = "1: An injected conversion has completed and its data may be read"]
    Set = 1,
}
impl From<JEOCF_A> for bool {
    #[inline(always)]
    fn from(variant: JEOCF_A) -> Self {
        variant as u8 != 0
    }
}
impl JEOCF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> JEOCF_A {
        match self.bits {
            false => JEOCF_A::Clear,
            true => JEOCF_A::Set,
        }
    }
    #[doc = "Checks if the value of the field is `Clear`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == JEOCF_A::Clear
    }
    #[doc = "Checks if the value of the field is `Set`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == JEOCF_A::Set
    }
}
#[doc = "Field `REOCF` reader - End of regular conversion flag"]
pub type REOCF_R = crate::BitReader<REOCF_A>;
#[doc = "End of regular conversion flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REOCF_A {
    #[doc = "0: No regular conversion has completed"]
    Clear = 0,
    #[doc = "1: A regular conversion has completed and its data may be read"]
    Set = 1,
}
impl From<REOCF_A> for bool {
    #[inline(always)]
    fn from(variant: REOCF_A) -> Self {
        variant as u8 != 0
    }
}
impl REOCF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REOCF_A {
        match self.bits {
            false => REOCF_A::Clear,
            true => REOCF_A::Set,
        }
    }
    #[doc = "Checks if the value of the field is `Clear`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == REOCF_A::Clear
    }
    #[doc = "Checks if the value of the field is `Set`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == REOCF_A::Set
    }
}
#[doc = "Field `JOVRF` reader - Injected conversion overrun flag"]
pub type JOVRF_R = crate::BitReader<JOVRF_A>;
#[doc = "Injected conversion overrun flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JOVRF_A {
    #[doc = "0: No injected conversion overrun has occurred"]
    Clear = 0,
    #[doc = "1: An injected conversion overrun has occurred, which means that an injected conversion finished while JEOCF was already ‘1’. JDATAR is not affected by overruns"]
    Set = 1,
}
impl From<JOVRF_A> for bool {
    #[inline(always)]
    fn from(variant: JOVRF_A) -> Self {
        variant as u8 != 0
    }
}
impl JOVRF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> JOVRF_A {
        match self.bits {
            false => JOVRF_A::Clear,
            true => JOVRF_A::Set,
        }
    }
    #[doc = "Checks if the value of the field is `Clear`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == JOVRF_A::Clear
    }
    #[doc = "Checks if the value of the field is `Set`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == JOVRF_A::Set
    }
}
#[doc = "Field `ROVRF` reader - Regular conversion overrun flag"]
pub type ROVRF_R = crate::BitReader<ROVRF_A>;
#[doc = "Regular conversion overrun flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ROVRF_A {
    #[doc = "0: No regular conversion overrun has occurred"]
    Clear = 0,
    #[doc = "1: A regular conversion overrun has occurred, which means that a regular conversion finished while REOCF was already ‘1’. RDATAR is not affected by overruns"]
    Set = 1,
}
impl From<ROVRF_A> for bool {
    #[inline(always)]
    fn from(variant: ROVRF_A) -> Self {
        variant as u8 != 0
    }
}
impl ROVRF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ROVRF_A {
        match self.bits {
            false => ROVRF_A::Clear,
            true => ROVRF_A::Set,
        }
    }
    #[doc = "Checks if the value of the field is `Clear`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == ROVRF_A::Clear
    }
    #[doc = "Checks if the value of the field is `Set`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == ROVRF_A::Set
    }
}
#[doc = "Field `AWDF` reader - Analog watchdog"]
pub type AWDF_R = crate::BitReader<AWDF_A>;
#[doc = "Analog watchdog\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWDF_A {
    #[doc = "0: No Analog watchdog event occurred"]
    Clear = 0,
    #[doc = "1: The analog watchdog block detected voltage which crosses the value programmed in the DFSDM_FLTxAWLTR or DFSDM_FLTxAWHTR registers"]
    Set = 1,
}
impl From<AWDF_A> for bool {
    #[inline(always)]
    fn from(variant: AWDF_A) -> Self {
        variant as u8 != 0
    }
}
impl AWDF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AWDF_A {
        match self.bits {
            false => AWDF_A::Clear,
            true => AWDF_A::Set,
        }
    }
    #[doc = "Checks if the value of the field is `Clear`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == AWDF_A::Clear
    }
    #[doc = "Checks if the value of the field is `Set`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == AWDF_A::Set
    }
}
#[doc = "Field `JCIP` reader - Injected conversion in progress status"]
pub type JCIP_R = crate::BitReader<JCIP_A>;
#[doc = "Injected conversion in progress status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JCIP_A {
    #[doc = "0: No request to convert the injected channel group (neither by software nor by trigger) has been issued"]
    NotInProgress = 0,
    #[doc = "1: The conversion of the injected channel group is in progress or a request for a injected conversion is pending, due either to ‘1’ being written to JSWSTART or to a trigger detection"]
    InProgress = 1,
}
impl From<JCIP_A> for bool {
    #[inline(always)]
    fn from(variant: JCIP_A) -> Self {
        variant as u8 != 0
    }
}
impl JCIP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> JCIP_A {
        match self.bits {
            false => JCIP_A::NotInProgress,
            true => JCIP_A::InProgress,
        }
    }
    #[doc = "Checks if the value of the field is `NotInProgress`"]
    #[inline(always)]
    pub fn is_not_in_progress(&self) -> bool {
        *self == JCIP_A::NotInProgress
    }
    #[doc = "Checks if the value of the field is `InProgress`"]
    #[inline(always)]
    pub fn is_in_progress(&self) -> bool {
        *self == JCIP_A::InProgress
    }
}
#[doc = "Field `RCIP` reader - Regular conversion in progress status"]
pub type RCIP_R = crate::BitReader<RCIP_A>;
#[doc = "Regular conversion in progress status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RCIP_A {
    #[doc = "0: No request to convert the regular channel has been issued"]
    NotInProgress = 0,
    #[doc = "1: The conversion of the regular channel is in progress or a request for a regular conversion is pending"]
    InProgress = 1,
}
impl From<RCIP_A> for bool {
    #[inline(always)]
    fn from(variant: RCIP_A) -> Self {
        variant as u8 != 0
    }
}
impl RCIP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RCIP_A {
        match self.bits {
            false => RCIP_A::NotInProgress,
            true => RCIP_A::InProgress,
        }
    }
    #[doc = "Checks if the value of the field is `NotInProgress`"]
    #[inline(always)]
    pub fn is_not_in_progress(&self) -> bool {
        *self == RCIP_A::NotInProgress
    }
    #[doc = "Checks if the value of the field is `InProgress`"]
    #[inline(always)]
    pub fn is_in_progress(&self) -> bool {
        *self == RCIP_A::InProgress
    }
}
#[doc = "Field `CKABF` reader - Clock absence flag"]
pub type CKABF_R = crate::FieldReader<u8, CKABF_A>;
#[doc = "Clock absence flag\n\nValue on reset: 255"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CKABF_A {
    #[doc = "0: Clock signal on channel y is present."]
    Clear = 0,
    #[doc = "1: Clock signal on channel y is not present"]
    Set = 1,
}
impl From<CKABF_A> for u8 {
    #[inline(always)]
    fn from(variant: CKABF_A) -> Self {
        variant as _
    }
}
impl CKABF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CKABF_A> {
        match self.bits {
            0 => Some(CKABF_A::Clear),
            1 => Some(CKABF_A::Set),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Clear`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == CKABF_A::Clear
    }
    #[doc = "Checks if the value of the field is `Set`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == CKABF_A::Set
    }
}
#[doc = "Field `SCDF` reader - short-circuit detector flag"]
pub type SCDF_R = crate::FieldReader<u8, SCDF_A>;
#[doc = "short-circuit detector flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SCDF_A {
    #[doc = "0: No short-circuit detector event occurred on channel y"]
    Clear = 0,
    #[doc = "1: The short-circuit detector counter reaches, on channel y, the value programmed in the DFSDM_CHyAWSCDR registers"]
    Set = 1,
}
impl From<SCDF_A> for u8 {
    #[inline(always)]
    fn from(variant: SCDF_A) -> Self {
        variant as _
    }
}
impl SCDF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SCDF_A> {
        match self.bits {
            0 => Some(SCDF_A::Clear),
            1 => Some(SCDF_A::Set),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Clear`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == SCDF_A::Clear
    }
    #[doc = "Checks if the value of the field is `Set`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == SCDF_A::Set
    }
}
impl R {
    #[doc = "Bit 0 - End of injected conversion flag"]
    #[inline(always)]
    pub fn jeocf(&self) -> JEOCF_R {
        JEOCF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - End of regular conversion flag"]
    #[inline(always)]
    pub fn reocf(&self) -> REOCF_R {
        REOCF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Injected conversion overrun flag"]
    #[inline(always)]
    pub fn jovrf(&self) -> JOVRF_R {
        JOVRF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Regular conversion overrun flag"]
    #[inline(always)]
    pub fn rovrf(&self) -> ROVRF_R {
        ROVRF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Analog watchdog"]
    #[inline(always)]
    pub fn awdf(&self) -> AWDF_R {
        AWDF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 13 - Injected conversion in progress status"]
    #[inline(always)]
    pub fn jcip(&self) -> JCIP_R {
        JCIP_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Regular conversion in progress status"]
    #[inline(always)]
    pub fn rcip(&self) -> RCIP_R {
        RCIP_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Clock absence flag"]
    #[inline(always)]
    pub fn ckabf(&self) -> CKABF_R {
        CKABF_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - short-circuit detector flag"]
    #[inline(always)]
    pub fn scdf(&self) -> SCDF_R {
        SCDF_R::new(((self.bits >> 24) & 0xff) as u8)
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
#[doc = "`reset()` method sets ISR to value 0x00ff_0000"]
impl crate::Resettable for ISR_SPEC {
    const RESET_VALUE: Self::Ux = 0x00ff_0000;
}
