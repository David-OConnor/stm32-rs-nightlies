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
#[doc = "Register `SR` writer"]
pub struct W(crate::W<SR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<SR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DRDY` reader - Data Ready"]
pub type DRDY_R = crate::BitReader<DRDY_A>;
#[doc = "Data Ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DRDY_A {
    #[doc = "0: The RNG_DR register is not yet valid, no random data is available"]
    Invalid = 0,
    #[doc = "1: The RNG_DR register contains valid random data"]
    Valid = 1,
}
impl From<DRDY_A> for bool {
    #[inline(always)]
    fn from(variant: DRDY_A) -> Self {
        variant as u8 != 0
    }
}
impl DRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DRDY_A {
        match self.bits {
            false => DRDY_A::Invalid,
            true => DRDY_A::Valid,
        }
    }
    #[doc = "Checks if the value of the field is `Invalid`"]
    #[inline(always)]
    pub fn is_invalid(&self) -> bool {
        *self == DRDY_A::Invalid
    }
    #[doc = "Checks if the value of the field is `Valid`"]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == DRDY_A::Valid
    }
}
#[doc = "Field `CECS` reader - Clock error current status"]
pub type CECS_R = crate::BitReader<CECSR_A>;
#[doc = "Clock error current status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CECSR_A {
    #[doc = "0: The RNG clock is correct (fRNGCLK> fHCLK/32)"]
    Correct = 0,
    #[doc = "1: The RNG clock before internal divider has been detected too slow (fRNGCLK< fHCLK/32)"]
    Slow = 1,
}
impl From<CECSR_A> for bool {
    #[inline(always)]
    fn from(variant: CECSR_A) -> Self {
        variant as u8 != 0
    }
}
impl CECS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CECSR_A {
        match self.bits {
            false => CECSR_A::Correct,
            true => CECSR_A::Slow,
        }
    }
    #[doc = "Checks if the value of the field is `Correct`"]
    #[inline(always)]
    pub fn is_correct(&self) -> bool {
        *self == CECSR_A::Correct
    }
    #[doc = "Checks if the value of the field is `Slow`"]
    #[inline(always)]
    pub fn is_slow(&self) -> bool {
        *self == CECSR_A::Slow
    }
}
#[doc = "Field `SECS` reader - Seed error current status"]
pub type SECS_R = crate::BitReader<SECSR_A>;
#[doc = "Seed error current status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SECSR_A {
    #[doc = "0: No faulty sequence has currently been detected. If the SEIS bit is set, this means that a faulty sequence was detected and the situation has been recovered"]
    NoFault = 0,
    #[doc = "1: At least one faulty sequence has been detected - see ref manual for details"]
    Fault = 1,
}
impl From<SECSR_A> for bool {
    #[inline(always)]
    fn from(variant: SECSR_A) -> Self {
        variant as u8 != 0
    }
}
impl SECS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SECSR_A {
        match self.bits {
            false => SECSR_A::NoFault,
            true => SECSR_A::Fault,
        }
    }
    #[doc = "Checks if the value of the field is `NoFault`"]
    #[inline(always)]
    pub fn is_no_fault(&self) -> bool {
        *self == SECSR_A::NoFault
    }
    #[doc = "Checks if the value of the field is `Fault`"]
    #[inline(always)]
    pub fn is_fault(&self) -> bool {
        *self == SECSR_A::Fault
    }
}
#[doc = "Field `CEIS` reader - Clock error interrupt status"]
pub type CEIS_R = crate::BitReader<CEIS_A>;
#[doc = "Clock error interrupt status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CEIS_A {
    #[doc = "0: The RNG clock is correct (fRNGCLK> fHCLK/32)"]
    Correct = 0,
    #[doc = "1: The RNG clock before internal divider has been detected too slow (fRNGCLK< fHCLK/32)"]
    Slow = 1,
}
impl From<CEIS_A> for bool {
    #[inline(always)]
    fn from(variant: CEIS_A) -> Self {
        variant as u8 != 0
    }
}
impl CEIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CEIS_A {
        match self.bits {
            false => CEIS_A::Correct,
            true => CEIS_A::Slow,
        }
    }
    #[doc = "Checks if the value of the field is `Correct`"]
    #[inline(always)]
    pub fn is_correct(&self) -> bool {
        *self == CEIS_A::Correct
    }
    #[doc = "Checks if the value of the field is `Slow`"]
    #[inline(always)]
    pub fn is_slow(&self) -> bool {
        *self == CEIS_A::Slow
    }
}
#[doc = "Field `CEIS` writer - Clock error interrupt status"]
pub type CEIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, CEIS_A, O>;
impl<'a, const O: u8> CEIS_W<'a, O> {
    #[doc = "The RNG clock is correct (fRNGCLK> fHCLK/32)"]
    #[inline(always)]
    pub fn correct(self) -> &'a mut W {
        self.variant(CEIS_A::Correct)
    }
    #[doc = "The RNG clock before internal divider has been detected too slow (fRNGCLK< fHCLK/32)"]
    #[inline(always)]
    pub fn slow(self) -> &'a mut W {
        self.variant(CEIS_A::Slow)
    }
}
#[doc = "Field `SEIS` reader - Seed error interrupt status"]
pub type SEIS_R = crate::BitReader<SEIS_A>;
#[doc = "Seed error interrupt status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SEIS_A {
    #[doc = "0: No faulty sequence detected"]
    NoFault = 0,
    #[doc = "1: At least one faulty sequence has been detected"]
    Fault = 1,
}
impl From<SEIS_A> for bool {
    #[inline(always)]
    fn from(variant: SEIS_A) -> Self {
        variant as u8 != 0
    }
}
impl SEIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEIS_A {
        match self.bits {
            false => SEIS_A::NoFault,
            true => SEIS_A::Fault,
        }
    }
    #[doc = "Checks if the value of the field is `NoFault`"]
    #[inline(always)]
    pub fn is_no_fault(&self) -> bool {
        *self == SEIS_A::NoFault
    }
    #[doc = "Checks if the value of the field is `Fault`"]
    #[inline(always)]
    pub fn is_fault(&self) -> bool {
        *self == SEIS_A::Fault
    }
}
#[doc = "Field `SEIS` writer - Seed error interrupt status"]
pub type SEIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, SEIS_A, O>;
impl<'a, const O: u8> SEIS_W<'a, O> {
    #[doc = "No faulty sequence detected"]
    #[inline(always)]
    pub fn no_fault(self) -> &'a mut W {
        self.variant(SEIS_A::NoFault)
    }
    #[doc = "At least one faulty sequence has been detected"]
    #[inline(always)]
    pub fn fault(self) -> &'a mut W {
        self.variant(SEIS_A::Fault)
    }
}
impl R {
    #[doc = "Bit 0 - Data Ready"]
    #[inline(always)]
    pub fn drdy(&self) -> DRDY_R {
        DRDY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clock error current status"]
    #[inline(always)]
    pub fn cecs(&self) -> CECS_R {
        CECS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Seed error current status"]
    #[inline(always)]
    pub fn secs(&self) -> SECS_R {
        SECS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - Clock error interrupt status"]
    #[inline(always)]
    pub fn ceis(&self) -> CEIS_R {
        CEIS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Seed error interrupt status"]
    #[inline(always)]
    pub fn seis(&self) -> SEIS_R {
        SEIS_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - Clock error interrupt status"]
    #[inline(always)]
    #[must_use]
    pub fn ceis(&mut self) -> CEIS_W<5> {
        CEIS_W::new(self)
    }
    #[doc = "Bit 6 - Seed error interrupt status"]
    #[inline(always)]
    #[must_use]
    pub fn seis(&mut self) -> SEIS_W<6> {
        SEIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](index.html) module"]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sr::R](R) reader structure"]
impl crate::Readable for SR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sr::W](W) writer structure"]
impl crate::Writable for SR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
