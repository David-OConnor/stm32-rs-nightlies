#[doc = "Register `CR` reader"]
pub struct R(crate::R<CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR` writer"]
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
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
impl From<crate::W<CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TPE` reader - Tamper pin enable"]
pub type TPE_R = crate::BitReader<TPE_A>;
#[doc = "Tamper pin enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TPE_A {
    #[doc = "0: The TAMPER pin is free for general purpose I/O"]
    General = 0,
    #[doc = "1: Tamper alternate I/O function is activated"]
    Alternate = 1,
}
impl From<TPE_A> for bool {
    #[inline(always)]
    fn from(variant: TPE_A) -> Self {
        variant as u8 != 0
    }
}
impl TPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TPE_A {
        match self.bits {
            false => TPE_A::General,
            true => TPE_A::Alternate,
        }
    }
    #[doc = "Checks if the value of the field is `General`"]
    #[inline(always)]
    pub fn is_general(&self) -> bool {
        *self == TPE_A::General
    }
    #[doc = "Checks if the value of the field is `Alternate`"]
    #[inline(always)]
    pub fn is_alternate(&self) -> bool {
        *self == TPE_A::Alternate
    }
}
#[doc = "Field `TPE` writer - Tamper pin enable"]
pub type TPE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, TPE_A, O>;
impl<'a, const O: u8> TPE_W<'a, O> {
    #[doc = "The TAMPER pin is free for general purpose I/O"]
    #[inline(always)]
    pub fn general(self) -> &'a mut W {
        self.variant(TPE_A::General)
    }
    #[doc = "Tamper alternate I/O function is activated"]
    #[inline(always)]
    pub fn alternate(self) -> &'a mut W {
        self.variant(TPE_A::Alternate)
    }
}
#[doc = "Field `TPAL` reader - Tamper pin active level"]
pub type TPAL_R = crate::BitReader<TPAL_A>;
#[doc = "Tamper pin active level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TPAL_A {
    #[doc = "0: A high level on the TAMPER pin resets all data backup registers (if TPE bit is set)"]
    High = 0,
    #[doc = "1: A low level on the TAMPER pin resets all data backup registers (if TPE bit is set)"]
    Low = 1,
}
impl From<TPAL_A> for bool {
    #[inline(always)]
    fn from(variant: TPAL_A) -> Self {
        variant as u8 != 0
    }
}
impl TPAL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TPAL_A {
        match self.bits {
            false => TPAL_A::High,
            true => TPAL_A::Low,
        }
    }
    #[doc = "Checks if the value of the field is `High`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == TPAL_A::High
    }
    #[doc = "Checks if the value of the field is `Low`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == TPAL_A::Low
    }
}
#[doc = "Field `TPAL` writer - Tamper pin active level"]
pub type TPAL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, TPAL_A, O>;
impl<'a, const O: u8> TPAL_W<'a, O> {
    #[doc = "A high level on the TAMPER pin resets all data backup registers (if TPE bit is set)"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(TPAL_A::High)
    }
    #[doc = "A low level on the TAMPER pin resets all data backup registers (if TPE bit is set)"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(TPAL_A::Low)
    }
}
impl R {
    #[doc = "Bit 0 - Tamper pin enable"]
    #[inline(always)]
    pub fn tpe(&self) -> TPE_R {
        TPE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Tamper pin active level"]
    #[inline(always)]
    pub fn tpal(&self) -> TPAL_R {
        TPAL_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Tamper pin enable"]
    #[inline(always)]
    #[must_use]
    pub fn tpe(&mut self) -> TPE_W<0> {
        TPE_W::new(self)
    }
    #[doc = "Bit 1 - Tamper pin active level"]
    #[inline(always)]
    #[must_use]
    pub fn tpal(&mut self) -> TPAL_W<1> {
        TPAL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Backup control register (BKP_CR)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](index.html) module"]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr::R](R) reader structure"]
impl crate::Readable for CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr::W](W) writer structure"]
impl crate::Writable for CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
