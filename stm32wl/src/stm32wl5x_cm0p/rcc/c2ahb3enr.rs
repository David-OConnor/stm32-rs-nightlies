#[doc = "Register `C2AHB3ENR` reader"]
pub struct R(crate::R<C2AHB3ENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C2AHB3ENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C2AHB3ENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C2AHB3ENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C2AHB3ENR` writer"]
pub struct W(crate::W<C2AHB3ENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C2AHB3ENR_SPEC>;
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
impl From<crate::W<C2AHB3ENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C2AHB3ENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PKAEN` reader - CPU2 PKA accelerator clock enable"]
pub type PKAEN_R = crate::BitReader<PKAEN_A>;
#[doc = "CPU2 PKA accelerator clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PKAEN_A {
    #[doc = "0: Clock disabled"]
    Disabled = 0,
    #[doc = "1: Clock enabled"]
    Enabled = 1,
}
impl From<PKAEN_A> for bool {
    #[inline(always)]
    fn from(variant: PKAEN_A) -> Self {
        variant as u8 != 0
    }
}
impl PKAEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PKAEN_A {
        match self.bits {
            false => PKAEN_A::Disabled,
            true => PKAEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PKAEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PKAEN_A::Enabled
    }
}
#[doc = "Field `PKAEN` writer - CPU2 PKA accelerator clock enable"]
pub type PKAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2AHB3ENR_SPEC, PKAEN_A, O>;
impl<'a, const O: u8> PKAEN_W<'a, O> {
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PKAEN_A::Disabled)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PKAEN_A::Enabled)
    }
}
#[doc = "Field `AESEN` reader - CPU2 AES accelerator clock enable"]
pub use PKAEN_R as AESEN_R;
#[doc = "Field `RNGEN` reader - CPU2 True RNG clocks enable"]
pub use PKAEN_R as RNGEN_R;
#[doc = "Field `HSEMEN` reader - CPU2 HSEM clock enable"]
pub use PKAEN_R as HSEMEN_R;
#[doc = "Field `IPCCEN` reader - CPU2 IPCC interface clock enable"]
pub use PKAEN_R as IPCCEN_R;
#[doc = "Field `FLASHEN` reader - CPU2 Flash interface clock enable"]
pub use PKAEN_R as FLASHEN_R;
#[doc = "Field `AESEN` writer - CPU2 AES accelerator clock enable"]
pub use PKAEN_W as AESEN_W;
#[doc = "Field `RNGEN` writer - CPU2 True RNG clocks enable"]
pub use PKAEN_W as RNGEN_W;
#[doc = "Field `HSEMEN` writer - CPU2 HSEM clock enable"]
pub use PKAEN_W as HSEMEN_W;
#[doc = "Field `IPCCEN` writer - CPU2 IPCC interface clock enable"]
pub use PKAEN_W as IPCCEN_W;
#[doc = "Field `FLASHEN` writer - CPU2 Flash interface clock enable"]
pub use PKAEN_W as FLASHEN_W;
impl R {
    #[doc = "Bit 16 - CPU2 PKA accelerator clock enable"]
    #[inline(always)]
    pub fn pkaen(&self) -> PKAEN_R {
        PKAEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - CPU2 AES accelerator clock enable"]
    #[inline(always)]
    pub fn aesen(&self) -> AESEN_R {
        AESEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - CPU2 True RNG clocks enable"]
    #[inline(always)]
    pub fn rngen(&self) -> RNGEN_R {
        RNGEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - CPU2 HSEM clock enable"]
    #[inline(always)]
    pub fn hsemen(&self) -> HSEMEN_R {
        HSEMEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - CPU2 IPCC interface clock enable"]
    #[inline(always)]
    pub fn ipccen(&self) -> IPCCEN_R {
        IPCCEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 25 - CPU2 Flash interface clock enable"]
    #[inline(always)]
    pub fn flashen(&self) -> FLASHEN_R {
        FLASHEN_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - CPU2 PKA accelerator clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn pkaen(&mut self) -> PKAEN_W<16> {
        PKAEN_W::new(self)
    }
    #[doc = "Bit 17 - CPU2 AES accelerator clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn aesen(&mut self) -> AESEN_W<17> {
        AESEN_W::new(self)
    }
    #[doc = "Bit 18 - CPU2 True RNG clocks enable"]
    #[inline(always)]
    #[must_use]
    pub fn rngen(&mut self) -> RNGEN_W<18> {
        RNGEN_W::new(self)
    }
    #[doc = "Bit 19 - CPU2 HSEM clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn hsemen(&mut self) -> HSEMEN_W<19> {
        HSEMEN_W::new(self)
    }
    #[doc = "Bit 20 - CPU2 IPCC interface clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn ipccen(&mut self) -> IPCCEN_W<20> {
        IPCCEN_W::new(self)
    }
    #[doc = "Bit 25 - CPU2 Flash interface clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn flashen(&mut self) -> FLASHEN_W<25> {
        FLASHEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CPU2 AHB3 peripheral clock enable register \\[dual core device only\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2ahb3enr](index.html) module"]
pub struct C2AHB3ENR_SPEC;
impl crate::RegisterSpec for C2AHB3ENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [c2ahb3enr::R](R) reader structure"]
impl crate::Readable for C2AHB3ENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c2ahb3enr::W](W) writer structure"]
impl crate::Writable for C2AHB3ENR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets C2AHB3ENR to value 0x0208_0000"]
impl crate::Resettable for C2AHB3ENR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0208_0000;
}
