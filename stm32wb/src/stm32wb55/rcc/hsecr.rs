#[doc = "Register `HSECR` reader"]
pub struct R(crate::R<HSECR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HSECR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HSECR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HSECR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HSECR` writer"]
pub struct W(crate::W<HSECR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HSECR_SPEC>;
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
impl From<crate::W<HSECR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HSECR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UNLOCKED` reader - Register lock system"]
pub type UNLOCKED_R = crate::BitReader<bool>;
#[doc = "Field `UNLOCKED` writer - Register lock system"]
pub type UNLOCKED_W<'a, const O: u8> = crate::BitWriter<'a, u32, HSECR_SPEC, bool, O>;
#[doc = "Field `HSES` reader - HSE Sense amplifier threshold"]
pub type HSES_R = crate::BitReader<bool>;
#[doc = "Field `HSES` writer - HSE Sense amplifier threshold"]
pub type HSES_W<'a, const O: u8> = crate::BitWriter<'a, u32, HSECR_SPEC, bool, O>;
#[doc = "Field `HSEGMC` reader - HSE current control"]
pub type HSEGMC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HSEGMC` writer - HSE current control"]
pub type HSEGMC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HSECR_SPEC, u8, u8, 3, O>;
#[doc = "Field `HSETUNE` reader - HSE capacitor tuning"]
pub type HSETUNE_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 0 - Register lock system"]
    #[inline(always)]
    pub fn unlocked(&self) -> UNLOCKED_R {
        UNLOCKED_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - HSE Sense amplifier threshold"]
    #[inline(always)]
    pub fn hses(&self) -> HSES_R {
        HSES_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - HSE current control"]
    #[inline(always)]
    pub fn hsegmc(&self) -> HSEGMC_R {
        HSEGMC_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:13 - HSE capacitor tuning"]
    #[inline(always)]
    pub fn hsetune(&self) -> HSETUNE_R {
        HSETUNE_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Register lock system"]
    #[inline(always)]
    #[must_use]
    pub fn unlocked(&mut self) -> UNLOCKED_W<0> {
        UNLOCKED_W::new(self)
    }
    #[doc = "Bit 3 - HSE Sense amplifier threshold"]
    #[inline(always)]
    #[must_use]
    pub fn hses(&mut self) -> HSES_W<3> {
        HSES_W::new(self)
    }
    #[doc = "Bits 4:6 - HSE current control"]
    #[inline(always)]
    #[must_use]
    pub fn hsegmc(&mut self) -> HSEGMC_W<4> {
        HSEGMC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock HSE register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hsecr](index.html) module"]
pub struct HSECR_SPEC;
impl crate::RegisterSpec for HSECR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hsecr::R](R) reader structure"]
impl crate::Readable for HSECR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hsecr::W](W) writer structure"]
impl crate::Writable for HSECR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HSECR to value 0x30"]
impl crate::Resettable for HSECR_SPEC {
    const RESET_VALUE: Self::Ux = 0x30;
}
