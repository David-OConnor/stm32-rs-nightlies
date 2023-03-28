#[doc = "Register `DMACRxCR` reader"]
pub struct R(crate::R<DMACRX_CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMACRX_CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMACRX_CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMACRX_CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMACRxCR` writer"]
pub struct W(crate::W<DMACRX_CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMACRX_CR_SPEC>;
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
impl From<crate::W<DMACRX_CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMACRX_CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SR` reader - Start or Stop Receive"]
pub type SR_R = crate::BitReader<bool>;
#[doc = "Field `SR` writer - Start or Stop Receive"]
pub type SR_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMACRX_CR_SPEC, bool, O>;
#[doc = "Field `RBSZ` reader - Receive Buffer size"]
pub type RBSZ_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RBSZ` writer - Receive Buffer size"]
pub type RBSZ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DMACRX_CR_SPEC, u16, u16, 14, O>;
#[doc = "Field `RXPBL` reader - Receive Programmable Burst Length"]
pub type RXPBL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RXPBL` writer - Receive Programmable Burst Length"]
pub type RXPBL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DMACRX_CR_SPEC, u8, u8, 6, O>;
#[doc = "Field `RPF` reader - DMA Rx Channel Packet Flush"]
pub type RPF_R = crate::BitReader<bool>;
#[doc = "Field `RPF` writer - DMA Rx Channel Packet Flush"]
pub type RPF_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMACRX_CR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Start or Stop Receive"]
    #[inline(always)]
    pub fn sr(&self) -> SR_R {
        SR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:14 - Receive Buffer size"]
    #[inline(always)]
    pub fn rbsz(&self) -> RBSZ_R {
        RBSZ_R::new(((self.bits >> 1) & 0x3fff) as u16)
    }
    #[doc = "Bits 16:21 - Receive Programmable Burst Length"]
    #[inline(always)]
    pub fn rxpbl(&self) -> RXPBL_R {
        RXPBL_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 31 - DMA Rx Channel Packet Flush"]
    #[inline(always)]
    pub fn rpf(&self) -> RPF_R {
        RPF_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Start or Stop Receive"]
    #[inline(always)]
    #[must_use]
    pub fn sr(&mut self) -> SR_W<0> {
        SR_W::new(self)
    }
    #[doc = "Bits 1:14 - Receive Buffer size"]
    #[inline(always)]
    #[must_use]
    pub fn rbsz(&mut self) -> RBSZ_W<1> {
        RBSZ_W::new(self)
    }
    #[doc = "Bits 16:21 - Receive Programmable Burst Length"]
    #[inline(always)]
    #[must_use]
    pub fn rxpbl(&mut self) -> RXPBL_W<16> {
        RXPBL_W::new(self)
    }
    #[doc = "Bit 31 - DMA Rx Channel Packet Flush"]
    #[inline(always)]
    #[must_use]
    pub fn rpf(&mut self) -> RPF_W<31> {
        RPF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel receive control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmacrx_cr](index.html) module"]
pub struct DMACRX_CR_SPEC;
impl crate::RegisterSpec for DMACRX_CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmacrx_cr::R](R) reader structure"]
impl crate::Readable for DMACRX_CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmacrx_cr::W](W) writer structure"]
impl crate::Writable for DMACRX_CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMACRxCR to value 0"]
impl crate::Resettable for DMACRX_CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
