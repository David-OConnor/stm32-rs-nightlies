#[doc = "Register `LPMCSR` reader"]
pub struct R(crate::R<LPMCSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LPMCSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LPMCSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LPMCSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LPMCSR` writer"]
pub struct W(crate::W<LPMCSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LPMCSR_SPEC>;
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
impl From<crate::W<LPMCSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LPMCSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LPMEN` reader - LPMEN"]
pub type LPMEN_R = crate::BitReader<LPMEN_A>;
#[doc = "LPMEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPMEN_A {
    #[doc = "0: enable the LPM support within the USB device"]
    Disabled = 0,
    #[doc = "1: no LPM transactions are handled"]
    Enabled = 1,
}
impl From<LPMEN_A> for bool {
    #[inline(always)]
    fn from(variant: LPMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl LPMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPMEN_A {
        match self.bits {
            false => LPMEN_A::Disabled,
            true => LPMEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LPMEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LPMEN_A::Enabled
    }
}
#[doc = "Field `LPMEN` writer - LPMEN"]
pub type LPMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPMCSR_SPEC, LPMEN_A, O>;
impl<'a, const O: u8> LPMEN_W<'a, O> {
    #[doc = "enable the LPM support within the USB device"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LPMEN_A::Disabled)
    }
    #[doc = "no LPM transactions are handled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LPMEN_A::Enabled)
    }
}
#[doc = "Field `LPMACK` reader - LPMACK"]
pub type LPMACK_R = crate::BitReader<LPMACK_A>;
#[doc = "LPMACK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPMACK_A {
    #[doc = "0: the valid LPM Token will be NYET"]
    Nyet = 0,
    #[doc = "1: the valid LPM Token will be ACK"]
    Ack = 1,
}
impl From<LPMACK_A> for bool {
    #[inline(always)]
    fn from(variant: LPMACK_A) -> Self {
        variant as u8 != 0
    }
}
impl LPMACK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPMACK_A {
        match self.bits {
            false => LPMACK_A::Nyet,
            true => LPMACK_A::Ack,
        }
    }
    #[doc = "Checks if the value of the field is `Nyet`"]
    #[inline(always)]
    pub fn is_nyet(&self) -> bool {
        *self == LPMACK_A::Nyet
    }
    #[doc = "Checks if the value of the field is `Ack`"]
    #[inline(always)]
    pub fn is_ack(&self) -> bool {
        *self == LPMACK_A::Ack
    }
}
#[doc = "Field `LPMACK` writer - LPMACK"]
pub type LPMACK_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPMCSR_SPEC, LPMACK_A, O>;
impl<'a, const O: u8> LPMACK_W<'a, O> {
    #[doc = "the valid LPM Token will be NYET"]
    #[inline(always)]
    pub fn nyet(self) -> &'a mut W {
        self.variant(LPMACK_A::Nyet)
    }
    #[doc = "the valid LPM Token will be ACK"]
    #[inline(always)]
    pub fn ack(self) -> &'a mut W {
        self.variant(LPMACK_A::Ack)
    }
}
#[doc = "Field `REMWAKE` reader - REMWAKE"]
pub type REMWAKE_R = crate::BitReader<bool>;
#[doc = "Field `BESL` reader - BESL"]
pub type BESL_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 0 - LPMEN"]
    #[inline(always)]
    pub fn lpmen(&self) -> LPMEN_R {
        LPMEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LPMACK"]
    #[inline(always)]
    pub fn lpmack(&self) -> LPMACK_R {
        LPMACK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - REMWAKE"]
    #[inline(always)]
    pub fn remwake(&self) -> REMWAKE_R {
        REMWAKE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - BESL"]
    #[inline(always)]
    pub fn besl(&self) -> BESL_R {
        BESL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - LPMEN"]
    #[inline(always)]
    #[must_use]
    pub fn lpmen(&mut self) -> LPMEN_W<0> {
        LPMEN_W::new(self)
    }
    #[doc = "Bit 1 - LPMACK"]
    #[inline(always)]
    #[must_use]
    pub fn lpmack(&mut self) -> LPMACK_W<1> {
        LPMACK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LPM control and status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lpmcsr](index.html) module"]
pub struct LPMCSR_SPEC;
impl crate::RegisterSpec for LPMCSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lpmcsr::R](R) reader structure"]
impl crate::Readable for LPMCSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lpmcsr::W](W) writer structure"]
impl crate::Writable for LPMCSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LPMCSR to value 0"]
impl crate::Resettable for LPMCSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
