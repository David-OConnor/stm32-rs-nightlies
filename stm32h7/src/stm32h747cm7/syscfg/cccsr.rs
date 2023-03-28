#[doc = "Register `CCCSR` reader"]
pub struct R(crate::R<CCCSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCCSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCCSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCCSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCCSR` writer"]
pub struct W(crate::W<CCCSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCCSR_SPEC>;
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
impl From<crate::W<CCCSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCCSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN` reader - enable"]
pub type EN_R = crate::BitReader<bool>;
#[doc = "Field `EN` writer - enable"]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCCSR_SPEC, bool, O>;
#[doc = "Field `CS` reader - Code selection"]
pub type CS_R = crate::BitReader<bool>;
#[doc = "Field `CS` writer - Code selection"]
pub type CS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCCSR_SPEC, bool, O>;
#[doc = "Field `READY` reader - Compensation cell ready flag"]
pub type READY_R = crate::BitReader<bool>;
#[doc = "Field `READY` writer - Compensation cell ready flag"]
pub type READY_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCCSR_SPEC, bool, O>;
#[doc = "Field `HSLV` reader - High-speed at low-voltage"]
pub type HSLV_R = crate::BitReader<bool>;
#[doc = "Field `HSLV` writer - High-speed at low-voltage"]
pub type HSLV_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCCSR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Code selection"]
    #[inline(always)]
    pub fn cs(&self) -> CS_R {
        CS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - Compensation cell ready flag"]
    #[inline(always)]
    pub fn ready(&self) -> READY_R {
        READY_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - High-speed at low-voltage"]
    #[inline(always)]
    pub fn hslv(&self) -> HSLV_R {
        HSLV_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - enable"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    #[doc = "Bit 1 - Code selection"]
    #[inline(always)]
    #[must_use]
    pub fn cs(&mut self) -> CS_W<1> {
        CS_W::new(self)
    }
    #[doc = "Bit 8 - Compensation cell ready flag"]
    #[inline(always)]
    #[must_use]
    pub fn ready(&mut self) -> READY_W<8> {
        READY_W::new(self)
    }
    #[doc = "Bit 16 - High-speed at low-voltage"]
    #[inline(always)]
    #[must_use]
    pub fn hslv(&mut self) -> HSLV_W<16> {
        HSLV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "compensation cell control/status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cccsr](index.html) module"]
pub struct CCCSR_SPEC;
impl crate::RegisterSpec for CCCSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cccsr::R](R) reader structure"]
impl crate::Readable for CCCSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cccsr::W](W) writer structure"]
impl crate::Writable for CCCSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CCCSR to value 0"]
impl crate::Resettable for CCCSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
