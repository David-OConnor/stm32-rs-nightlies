#[doc = "Register `RCC_ETHCKSELR` reader"]
pub struct R(crate::R<RCC_ETHCKSELR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_ETHCKSELR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_ETHCKSELR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_ETHCKSELR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_ETHCKSELR` writer"]
pub struct W(crate::W<RCC_ETHCKSELR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_ETHCKSELR_SPEC>;
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
impl From<crate::W<RCC_ETHCKSELR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_ETHCKSELR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ETHSRC` reader - ETHSRC"]
pub type ETHSRC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ETHSRC` writer - ETHSRC"]
pub type ETHSRC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RCC_ETHCKSELR_SPEC, u8, u8, 2, O>;
#[doc = "Field `ETHPTPDIV` reader - ETHPTPDIV"]
pub type ETHPTPDIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ETHPTPDIV` writer - ETHPTPDIV"]
pub type ETHPTPDIV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RCC_ETHCKSELR_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:1 - ETHSRC"]
    #[inline(always)]
    pub fn ethsrc(&self) -> ETHSRC_R {
        ETHSRC_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:7 - ETHPTPDIV"]
    #[inline(always)]
    pub fn ethptpdiv(&self) -> ETHPTPDIV_R {
        ETHPTPDIV_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - ETHSRC"]
    #[inline(always)]
    #[must_use]
    pub fn ethsrc(&mut self) -> ETHSRC_W<0> {
        ETHSRC_W::new(self)
    }
    #[doc = "Bits 4:7 - ETHPTPDIV"]
    #[inline(always)]
    #[must_use]
    pub fn ethptpdiv(&mut self) -> ETHPTPDIV_W<4> {
        ETHPTPDIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used to control the selection of the kernel clock for the ETH block. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_ethckselr](index.html) module"]
pub struct RCC_ETHCKSELR_SPEC;
impl crate::RegisterSpec for RCC_ETHCKSELR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_ethckselr::R](R) reader structure"]
impl crate::Readable for RCC_ETHCKSELR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_ethckselr::W](W) writer structure"]
impl crate::Writable for RCC_ETHCKSELR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RCC_ETHCKSELR to value 0"]
impl crate::Resettable for RCC_ETHCKSELR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
