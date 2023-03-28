#[doc = "Register `ETH_MACECR` reader"]
pub struct R(crate::R<ETH_MACECR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_MACECR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_MACECR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_MACECR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETH_MACECR` writer"]
pub struct W(crate::W<ETH_MACECR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETH_MACECR_SPEC>;
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
impl From<crate::W<ETH_MACECR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETH_MACECR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPSL` reader - GPSL"]
pub type GPSL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `GPSL` writer - GPSL"]
pub type GPSL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ETH_MACECR_SPEC, u16, u16, 14, O>;
#[doc = "Field `DCRCC` reader - DCRCC"]
pub type DCRCC_R = crate::BitReader<bool>;
#[doc = "Field `DCRCC` writer - DCRCC"]
pub type DCRCC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACECR_SPEC, bool, O>;
#[doc = "Field `SPEN` reader - SPEN"]
pub type SPEN_R = crate::BitReader<bool>;
#[doc = "Field `SPEN` writer - SPEN"]
pub type SPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACECR_SPEC, bool, O>;
#[doc = "Field `USP` reader - USP"]
pub type USP_R = crate::BitReader<bool>;
#[doc = "Field `USP` writer - USP"]
pub type USP_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACECR_SPEC, bool, O>;
#[doc = "Field `EIPGEN` reader - EIPGEN"]
pub type EIPGEN_R = crate::BitReader<bool>;
#[doc = "Field `EIPGEN` writer - EIPGEN"]
pub type EIPGEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACECR_SPEC, bool, O>;
#[doc = "Field `EIPG` reader - EIPG"]
pub type EIPG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EIPG` writer - EIPG"]
pub type EIPG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ETH_MACECR_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:13 - GPSL"]
    #[inline(always)]
    pub fn gpsl(&self) -> GPSL_R {
        GPSL_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bit 16 - DCRCC"]
    #[inline(always)]
    pub fn dcrcc(&self) -> DCRCC_R {
        DCRCC_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - SPEN"]
    #[inline(always)]
    pub fn spen(&self) -> SPEN_R {
        SPEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - USP"]
    #[inline(always)]
    pub fn usp(&self) -> USP_R {
        USP_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 24 - EIPGEN"]
    #[inline(always)]
    pub fn eipgen(&self) -> EIPGEN_R {
        EIPGEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:29 - EIPG"]
    #[inline(always)]
    pub fn eipg(&self) -> EIPG_R {
        EIPG_R::new(((self.bits >> 25) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:13 - GPSL"]
    #[inline(always)]
    #[must_use]
    pub fn gpsl(&mut self) -> GPSL_W<0> {
        GPSL_W::new(self)
    }
    #[doc = "Bit 16 - DCRCC"]
    #[inline(always)]
    #[must_use]
    pub fn dcrcc(&mut self) -> DCRCC_W<16> {
        DCRCC_W::new(self)
    }
    #[doc = "Bit 17 - SPEN"]
    #[inline(always)]
    #[must_use]
    pub fn spen(&mut self) -> SPEN_W<17> {
        SPEN_W::new(self)
    }
    #[doc = "Bit 18 - USP"]
    #[inline(always)]
    #[must_use]
    pub fn usp(&mut self) -> USP_W<18> {
        USP_W::new(self)
    }
    #[doc = "Bit 24 - EIPGEN"]
    #[inline(always)]
    #[must_use]
    pub fn eipgen(&mut self) -> EIPGEN_W<24> {
        EIPGEN_W::new(self)
    }
    #[doc = "Bits 25:29 - EIPG"]
    #[inline(always)]
    #[must_use]
    pub fn eipg(&mut self) -> EIPG_W<25> {
        EIPG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The MAC Extended Configuration Register establishes the operating mode of the MAC.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_macecr](index.html) module"]
pub struct ETH_MACECR_SPEC;
impl crate::RegisterSpec for ETH_MACECR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_macecr::R](R) reader structure"]
impl crate::Readable for ETH_MACECR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eth_macecr::W](W) writer structure"]
impl crate::Writable for ETH_MACECR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ETH_MACECR to value 0"]
impl crate::Resettable for ETH_MACECR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
