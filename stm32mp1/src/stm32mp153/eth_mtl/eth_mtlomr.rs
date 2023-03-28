#[doc = "Register `ETH_MTLOMR` reader"]
pub struct R(crate::R<ETH_MTLOMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_MTLOMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_MTLOMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_MTLOMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETH_MTLOMR` writer"]
pub struct W(crate::W<ETH_MTLOMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETH_MTLOMR_SPEC>;
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
impl From<crate::W<ETH_MTLOMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETH_MTLOMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DTXSTS` reader - DTXSTS"]
pub type DTXSTS_R = crate::BitReader<bool>;
#[doc = "Field `DTXSTS` writer - DTXSTS"]
pub type DTXSTS_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MTLOMR_SPEC, bool, O>;
#[doc = "Field `RAA` reader - RAA"]
pub type RAA_R = crate::BitReader<bool>;
#[doc = "Field `RAA` writer - RAA"]
pub type RAA_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MTLOMR_SPEC, bool, O>;
#[doc = "Field `SCHALG` reader - SCHALG"]
pub type SCHALG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SCHALG` writer - SCHALG"]
pub type SCHALG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ETH_MTLOMR_SPEC, u8, u8, 2, O>;
#[doc = "Field `CNTPRST` reader - CNTPRST"]
pub type CNTPRST_R = crate::BitReader<bool>;
#[doc = "Field `CNTPRST` writer - CNTPRST"]
pub type CNTPRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MTLOMR_SPEC, bool, O>;
#[doc = "Field `CNTCLR` reader - CNTCLR"]
pub type CNTCLR_R = crate::BitReader<bool>;
#[doc = "Field `CNTCLR` writer - CNTCLR"]
pub type CNTCLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MTLOMR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 1 - DTXSTS"]
    #[inline(always)]
    pub fn dtxsts(&self) -> DTXSTS_R {
        DTXSTS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RAA"]
    #[inline(always)]
    pub fn raa(&self) -> RAA_R {
        RAA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 5:6 - SCHALG"]
    #[inline(always)]
    pub fn schalg(&self) -> SCHALG_R {
        SCHALG_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 8 - CNTPRST"]
    #[inline(always)]
    pub fn cntprst(&self) -> CNTPRST_R {
        CNTPRST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CNTCLR"]
    #[inline(always)]
    pub fn cntclr(&self) -> CNTCLR_R {
        CNTCLR_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - DTXSTS"]
    #[inline(always)]
    #[must_use]
    pub fn dtxsts(&mut self) -> DTXSTS_W<1> {
        DTXSTS_W::new(self)
    }
    #[doc = "Bit 2 - RAA"]
    #[inline(always)]
    #[must_use]
    pub fn raa(&mut self) -> RAA_W<2> {
        RAA_W::new(self)
    }
    #[doc = "Bits 5:6 - SCHALG"]
    #[inline(always)]
    #[must_use]
    pub fn schalg(&mut self) -> SCHALG_W<5> {
        SCHALG_W::new(self)
    }
    #[doc = "Bit 8 - CNTPRST"]
    #[inline(always)]
    #[must_use]
    pub fn cntprst(&mut self) -> CNTPRST_W<8> {
        CNTPRST_W::new(self)
    }
    #[doc = "Bit 9 - CNTCLR"]
    #[inline(always)]
    #[must_use]
    pub fn cntclr(&mut self) -> CNTCLR_W<9> {
        CNTCLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The Operating Mode register establishes the Transmit and Receive operating modes and commands.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_mtlomr](index.html) module"]
pub struct ETH_MTLOMR_SPEC;
impl crate::RegisterSpec for ETH_MTLOMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_mtlomr::R](R) reader structure"]
impl crate::Readable for ETH_MTLOMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eth_mtlomr::W](W) writer structure"]
impl crate::Writable for ETH_MTLOMR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ETH_MTLOMR to value 0"]
impl crate::Resettable for ETH_MTLOMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
