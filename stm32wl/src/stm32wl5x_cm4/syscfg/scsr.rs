#[doc = "Register `SCSR` reader"]
pub struct R(crate::R<SCSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCSR` writer"]
pub struct W(crate::W<SCSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCSR_SPEC>;
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
impl From<crate::W<SCSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SRAM2ER` reader - SRAM2 erase"]
pub type SRAM2ER_R = crate::BitReader<SRAM2ERW_A>;
#[doc = "SRAM2 erase\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM2ERW_A {
    #[doc = "1: Start SRAM2 erase operation"]
    Erase = 1,
}
impl From<SRAM2ERW_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM2ERW_A) -> Self {
        variant as u8 != 0
    }
}
impl SRAM2ER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SRAM2ERW_A> {
        match self.bits {
            true => Some(SRAM2ERW_A::Erase),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Erase`"]
    #[inline(always)]
    pub fn is_erase(&self) -> bool {
        *self == SRAM2ERW_A::Erase
    }
}
#[doc = "Field `SRAM2ER` writer - SRAM2 erase"]
pub type SRAM2ER_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCSR_SPEC, SRAM2ERW_A, O>;
impl<'a, const O: u8> SRAM2ER_W<'a, O> {
    #[doc = "Start SRAM2 erase operation"]
    #[inline(always)]
    pub fn erase(self) -> &'a mut W {
        self.variant(SRAM2ERW_A::Erase)
    }
}
#[doc = "Field `SRAMBSY` reader - SRAM1, SRAM2 and PKA SRAM busy by erase operation"]
pub type SRAMBSY_R = crate::BitReader<SRAMBSY_A>;
#[doc = "SRAM1, SRAM2 and PKA SRAM busy by erase operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAMBSY_A {
    #[doc = "0: No SRAM1 or SRAM2 erase operation is ongoing"]
    Idle = 0,
    #[doc = "1: SRAM1 or SRAM2 erase operation is ongoing"]
    Busy = 1,
}
impl From<SRAMBSY_A> for bool {
    #[inline(always)]
    fn from(variant: SRAMBSY_A) -> Self {
        variant as u8 != 0
    }
}
impl SRAMBSY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAMBSY_A {
        match self.bits {
            false => SRAMBSY_A::Idle,
            true => SRAMBSY_A::Busy,
        }
    }
    #[doc = "Checks if the value of the field is `Idle`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == SRAMBSY_A::Idle
    }
    #[doc = "Checks if the value of the field is `Busy`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == SRAMBSY_A::Busy
    }
}
#[doc = "Field `PKASRAMBSY` reader - PKA SRAM busy by erase operation"]
pub type PKASRAMBSY_R = crate::BitReader<PKASRAMBSY_A>;
#[doc = "PKA SRAM busy by erase operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PKASRAMBSY_A {
    #[doc = "0: No PKA SRAM erase operation is ongoing"]
    Idle = 0,
    #[doc = "1: PKA SRAM erase operation is ongoing"]
    Busy = 1,
}
impl From<PKASRAMBSY_A> for bool {
    #[inline(always)]
    fn from(variant: PKASRAMBSY_A) -> Self {
        variant as u8 != 0
    }
}
impl PKASRAMBSY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PKASRAMBSY_A {
        match self.bits {
            false => PKASRAMBSY_A::Idle,
            true => PKASRAMBSY_A::Busy,
        }
    }
    #[doc = "Checks if the value of the field is `Idle`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == PKASRAMBSY_A::Idle
    }
    #[doc = "Checks if the value of the field is `Busy`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == PKASRAMBSY_A::Busy
    }
}
impl R {
    #[doc = "Bit 0 - SRAM2 erase"]
    #[inline(always)]
    pub fn sram2er(&self) -> SRAM2ER_R {
        SRAM2ER_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SRAM1, SRAM2 and PKA SRAM busy by erase operation"]
    #[inline(always)]
    pub fn srambsy(&self) -> SRAMBSY_R {
        SRAMBSY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - PKA SRAM busy by erase operation"]
    #[inline(always)]
    pub fn pkasrambsy(&self) -> PKASRAMBSY_R {
        PKASRAMBSY_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SRAM2 erase"]
    #[inline(always)]
    #[must_use]
    pub fn sram2er(&mut self) -> SRAM2ER_W<0> {
        SRAM2ER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SCSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scsr](index.html) module"]
pub struct SCSR_SPEC;
impl crate::RegisterSpec for SCSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scsr::R](R) reader structure"]
impl crate::Readable for SCSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scsr::W](W) writer structure"]
impl crate::Writable for SCSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCSR to value 0"]
impl crate::Resettable for SCSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
