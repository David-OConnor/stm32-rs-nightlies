#[doc = "Register `RCC_MP_APRSTCR` reader"]
pub struct R(crate::R<RCC_MP_APRSTCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_MP_APRSTCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_MP_APRSTCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_MP_APRSTCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_MP_APRSTCR` writer"]
pub struct W(crate::W<RCC_MP_APRSTCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_MP_APRSTCR_SPEC>;
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
impl From<crate::W<RCC_MP_APRSTCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_MP_APRSTCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RDCTLEN` reader - RDCTLEN"]
pub type RDCTLEN_R = crate::BitReader<bool>;
#[doc = "Field `RDCTLEN` writer - RDCTLEN"]
pub type RDCTLEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MP_APRSTCR_SPEC, bool, O>;
#[doc = "Field `RSTTO` reader - RSTTO"]
pub type RSTTO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RSTTO` writer - RSTTO"]
pub type RSTTO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RCC_MP_APRSTCR_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bit 0 - RDCTLEN"]
    #[inline(always)]
    pub fn rdctlen(&self) -> RDCTLEN_R {
        RDCTLEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:14 - RSTTO"]
    #[inline(always)]
    pub fn rstto(&self) -> RSTTO_R {
        RSTTO_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - RDCTLEN"]
    #[inline(always)]
    #[must_use]
    pub fn rdctlen(&mut self) -> RDCTLEN_W<0> {
        RDCTLEN_W::new(self)
    }
    #[doc = "Bits 8:14 - RSTTO"]
    #[inline(always)]
    #[must_use]
    pub fn rstto(&mut self) -> RSTTO_W<8> {
        RSTTO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used to control the behavior of the warm reset. If TZEN = , this register can only be modified in secure mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mp_aprstcr](index.html) module"]
pub struct RCC_MP_APRSTCR_SPEC;
impl crate::RegisterSpec for RCC_MP_APRSTCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_mp_aprstcr::R](R) reader structure"]
impl crate::Readable for RCC_MP_APRSTCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_mp_aprstcr::W](W) writer structure"]
impl crate::Writable for RCC_MP_APRSTCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RCC_MP_APRSTCR to value 0x7f00"]
impl crate::Resettable for RCC_MP_APRSTCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x7f00;
}
