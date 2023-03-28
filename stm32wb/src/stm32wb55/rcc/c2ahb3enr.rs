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
#[doc = "Field `PKAEN` reader - CPU2 PKAEN"]
pub type PKAEN_R = crate::BitReader<bool>;
#[doc = "Field `PKAEN` writer - CPU2 PKAEN"]
pub type PKAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2AHB3ENR_SPEC, bool, O>;
#[doc = "Field `AES2EN` reader - CPU2 AES2EN"]
pub type AES2EN_R = crate::BitReader<bool>;
#[doc = "Field `AES2EN` writer - CPU2 AES2EN"]
pub type AES2EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2AHB3ENR_SPEC, bool, O>;
#[doc = "Field `RNGEN` reader - CPU2 RNGEN"]
pub type RNGEN_R = crate::BitReader<bool>;
#[doc = "Field `RNGEN` writer - CPU2 RNGEN"]
pub type RNGEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2AHB3ENR_SPEC, bool, O>;
#[doc = "Field `HSEMEN` reader - CPU2 HSEMEN"]
pub type HSEMEN_R = crate::BitReader<bool>;
#[doc = "Field `HSEMEN` writer - CPU2 HSEMEN"]
pub type HSEMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2AHB3ENR_SPEC, bool, O>;
#[doc = "Field `IPCCEN` reader - CPU2 IPCCEN"]
pub type IPCCEN_R = crate::BitReader<bool>;
#[doc = "Field `IPCCEN` writer - CPU2 IPCCEN"]
pub type IPCCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2AHB3ENR_SPEC, bool, O>;
#[doc = "Field `FLASHEN` reader - CPU2 FLASHEN"]
pub type FLASHEN_R = crate::BitReader<bool>;
#[doc = "Field `FLASHEN` writer - CPU2 FLASHEN"]
pub type FLASHEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2AHB3ENR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 16 - CPU2 PKAEN"]
    #[inline(always)]
    pub fn pkaen(&self) -> PKAEN_R {
        PKAEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - CPU2 AES2EN"]
    #[inline(always)]
    pub fn aes2en(&self) -> AES2EN_R {
        AES2EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - CPU2 RNGEN"]
    #[inline(always)]
    pub fn rngen(&self) -> RNGEN_R {
        RNGEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - CPU2 HSEMEN"]
    #[inline(always)]
    pub fn hsemen(&self) -> HSEMEN_R {
        HSEMEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - CPU2 IPCCEN"]
    #[inline(always)]
    pub fn ipccen(&self) -> IPCCEN_R {
        IPCCEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 25 - CPU2 FLASHEN"]
    #[inline(always)]
    pub fn flashen(&self) -> FLASHEN_R {
        FLASHEN_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - CPU2 PKAEN"]
    #[inline(always)]
    #[must_use]
    pub fn pkaen(&mut self) -> PKAEN_W<16> {
        PKAEN_W::new(self)
    }
    #[doc = "Bit 17 - CPU2 AES2EN"]
    #[inline(always)]
    #[must_use]
    pub fn aes2en(&mut self) -> AES2EN_W<17> {
        AES2EN_W::new(self)
    }
    #[doc = "Bit 18 - CPU2 RNGEN"]
    #[inline(always)]
    #[must_use]
    pub fn rngen(&mut self) -> RNGEN_W<18> {
        RNGEN_W::new(self)
    }
    #[doc = "Bit 19 - CPU2 HSEMEN"]
    #[inline(always)]
    #[must_use]
    pub fn hsemen(&mut self) -> HSEMEN_W<19> {
        HSEMEN_W::new(self)
    }
    #[doc = "Bit 20 - CPU2 IPCCEN"]
    #[inline(always)]
    #[must_use]
    pub fn ipccen(&mut self) -> IPCCEN_W<20> {
        IPCCEN_W::new(self)
    }
    #[doc = "Bit 25 - CPU2 FLASHEN"]
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
#[doc = "CPU2 AHB3 peripheral clock enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2ahb3enr](index.html) module"]
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
