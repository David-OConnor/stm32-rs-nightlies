#[doc = "Register `RCC_MC_APB4LPENCLRR` reader"]
pub struct R(crate::R<RCC_MC_APB4LPENCLRR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_MC_APB4LPENCLRR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_MC_APB4LPENCLRR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_MC_APB4LPENCLRR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_MC_APB4LPENCLRR` writer"]
pub struct W(crate::W<RCC_MC_APB4LPENCLRR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_MC_APB4LPENCLRR_SPEC>;
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
impl From<crate::W<RCC_MC_APB4LPENCLRR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_MC_APB4LPENCLRR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LTDCLPEN` reader - LTDCLPEN"]
pub type LTDCLPEN_R = crate::BitReader<bool>;
#[doc = "Field `LTDCLPEN` writer - LTDCLPEN"]
pub type LTDCLPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MC_APB4LPENCLRR_SPEC, bool, O>;
#[doc = "Field `DSILPEN` reader - DSILPEN"]
pub type DSILPEN_R = crate::BitReader<bool>;
#[doc = "Field `DSILPEN` writer - DSILPEN"]
pub type DSILPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MC_APB4LPENCLRR_SPEC, bool, O>;
#[doc = "Field `DDRPERFMLPEN` reader - DDRPERFMLPEN"]
pub type DDRPERFMLPEN_R = crate::BitReader<bool>;
#[doc = "Field `DDRPERFMLPEN` writer - DDRPERFMLPEN"]
pub type DDRPERFMLPEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RCC_MC_APB4LPENCLRR_SPEC, bool, O>;
#[doc = "Field `USBPHYLPEN` reader - USBPHYLPEN"]
pub type USBPHYLPEN_R = crate::BitReader<bool>;
#[doc = "Field `USBPHYLPEN` writer - USBPHYLPEN"]
pub type USBPHYLPEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RCC_MC_APB4LPENCLRR_SPEC, bool, O>;
#[doc = "Field `STGENROLPEN` reader - STGENROLPEN"]
pub type STGENROLPEN_R = crate::BitReader<bool>;
#[doc = "Field `STGENROLPEN` writer - STGENROLPEN"]
pub type STGENROLPEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RCC_MC_APB4LPENCLRR_SPEC, bool, O>;
#[doc = "Field `STGENROSTPEN` reader - STGENROSTPEN"]
pub type STGENROSTPEN_R = crate::BitReader<bool>;
#[doc = "Field `STGENROSTPEN` writer - STGENROSTPEN"]
pub type STGENROSTPEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RCC_MC_APB4LPENCLRR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - LTDCLPEN"]
    #[inline(always)]
    pub fn ltdclpen(&self) -> LTDCLPEN_R {
        LTDCLPEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - DSILPEN"]
    #[inline(always)]
    pub fn dsilpen(&self) -> DSILPEN_R {
        DSILPEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - DDRPERFMLPEN"]
    #[inline(always)]
    pub fn ddrperfmlpen(&self) -> DDRPERFMLPEN_R {
        DDRPERFMLPEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - USBPHYLPEN"]
    #[inline(always)]
    pub fn usbphylpen(&self) -> USBPHYLPEN_R {
        USBPHYLPEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - STGENROLPEN"]
    #[inline(always)]
    pub fn stgenrolpen(&self) -> STGENROLPEN_R {
        STGENROLPEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - STGENROSTPEN"]
    #[inline(always)]
    pub fn stgenrostpen(&self) -> STGENROSTPEN_R {
        STGENROSTPEN_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LTDCLPEN"]
    #[inline(always)]
    #[must_use]
    pub fn ltdclpen(&mut self) -> LTDCLPEN_W<0> {
        LTDCLPEN_W::new(self)
    }
    #[doc = "Bit 4 - DSILPEN"]
    #[inline(always)]
    #[must_use]
    pub fn dsilpen(&mut self) -> DSILPEN_W<4> {
        DSILPEN_W::new(self)
    }
    #[doc = "Bit 8 - DDRPERFMLPEN"]
    #[inline(always)]
    #[must_use]
    pub fn ddrperfmlpen(&mut self) -> DDRPERFMLPEN_W<8> {
        DDRPERFMLPEN_W::new(self)
    }
    #[doc = "Bit 16 - USBPHYLPEN"]
    #[inline(always)]
    #[must_use]
    pub fn usbphylpen(&mut self) -> USBPHYLPEN_W<16> {
        USBPHYLPEN_W::new(self)
    }
    #[doc = "Bit 20 - STGENROLPEN"]
    #[inline(always)]
    #[must_use]
    pub fn stgenrolpen(&mut self) -> STGENROLPEN_W<20> {
        STGENROLPEN_W::new(self)
    }
    #[doc = "Bit 21 - STGENROSTPEN"]
    #[inline(always)]
    #[must_use]
    pub fn stgenrostpen(&mut self) -> STGENROSTPEN_W<21> {
        STGENROSTPEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used by the MCU in order to clear the PERxLPEN bit\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mc_apb4lpenclrr](index.html) module"]
pub struct RCC_MC_APB4LPENCLRR_SPEC;
impl crate::RegisterSpec for RCC_MC_APB4LPENCLRR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_mc_apb4lpenclrr::R](R) reader structure"]
impl crate::Readable for RCC_MC_APB4LPENCLRR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_mc_apb4lpenclrr::W](W) writer structure"]
impl crate::Writable for RCC_MC_APB4LPENCLRR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RCC_MC_APB4LPENCLRR to value 0x0011_0111"]
impl crate::Resettable for RCC_MC_APB4LPENCLRR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0011_0111;
}
