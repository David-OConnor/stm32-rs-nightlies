#[doc = "Register `ETH_MACPOCR` reader"]
pub struct R(crate::R<ETH_MACPOCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_MACPOCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_MACPOCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_MACPOCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETH_MACPOCR` writer"]
pub struct W(crate::W<ETH_MACPOCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETH_MACPOCR_SPEC>;
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
impl From<crate::W<ETH_MACPOCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETH_MACPOCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PTOEN` reader - PTOEN"]
pub type PTOEN_R = crate::BitReader<bool>;
#[doc = "Field `PTOEN` writer - PTOEN"]
pub type PTOEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACPOCR_SPEC, bool, O>;
#[doc = "Field `ASYNCEN` reader - ASYNCEN"]
pub type ASYNCEN_R = crate::BitReader<bool>;
#[doc = "Field `ASYNCEN` writer - ASYNCEN"]
pub type ASYNCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACPOCR_SPEC, bool, O>;
#[doc = "Field `APDREQEN` reader - APDREQEN"]
pub type APDREQEN_R = crate::BitReader<bool>;
#[doc = "Field `APDREQEN` writer - APDREQEN"]
pub type APDREQEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACPOCR_SPEC, bool, O>;
#[doc = "Field `ASYNCTRIG` reader - ASYNCTRIG"]
pub type ASYNCTRIG_R = crate::BitReader<bool>;
#[doc = "Field `ASYNCTRIG` writer - ASYNCTRIG"]
pub type ASYNCTRIG_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACPOCR_SPEC, bool, O>;
#[doc = "Field `APDREQTRIG` reader - APDREQTRIG"]
pub type APDREQTRIG_R = crate::BitReader<bool>;
#[doc = "Field `APDREQTRIG` writer - APDREQTRIG"]
pub type APDREQTRIG_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACPOCR_SPEC, bool, O>;
#[doc = "Field `DRRDIS` reader - DRRDIS"]
pub type DRRDIS_R = crate::BitReader<bool>;
#[doc = "Field `DRRDIS` writer - DRRDIS"]
pub type DRRDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACPOCR_SPEC, bool, O>;
#[doc = "Field `DN` reader - DN"]
pub type DN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DN` writer - DN"]
pub type DN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ETH_MACPOCR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bit 0 - PTOEN"]
    #[inline(always)]
    pub fn ptoen(&self) -> PTOEN_R {
        PTOEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ASYNCEN"]
    #[inline(always)]
    pub fn asyncen(&self) -> ASYNCEN_R {
        ASYNCEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - APDREQEN"]
    #[inline(always)]
    pub fn apdreqen(&self) -> APDREQEN_R {
        APDREQEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - ASYNCTRIG"]
    #[inline(always)]
    pub fn asynctrig(&self) -> ASYNCTRIG_R {
        ASYNCTRIG_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - APDREQTRIG"]
    #[inline(always)]
    pub fn apdreqtrig(&self) -> APDREQTRIG_R {
        APDREQTRIG_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - DRRDIS"]
    #[inline(always)]
    pub fn drrdis(&self) -> DRRDIS_R {
        DRRDIS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:15 - DN"]
    #[inline(always)]
    pub fn dn(&self) -> DN_R {
        DN_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - PTOEN"]
    #[inline(always)]
    #[must_use]
    pub fn ptoen(&mut self) -> PTOEN_W<0> {
        PTOEN_W::new(self)
    }
    #[doc = "Bit 1 - ASYNCEN"]
    #[inline(always)]
    #[must_use]
    pub fn asyncen(&mut self) -> ASYNCEN_W<1> {
        ASYNCEN_W::new(self)
    }
    #[doc = "Bit 2 - APDREQEN"]
    #[inline(always)]
    #[must_use]
    pub fn apdreqen(&mut self) -> APDREQEN_W<2> {
        APDREQEN_W::new(self)
    }
    #[doc = "Bit 4 - ASYNCTRIG"]
    #[inline(always)]
    #[must_use]
    pub fn asynctrig(&mut self) -> ASYNCTRIG_W<4> {
        ASYNCTRIG_W::new(self)
    }
    #[doc = "Bit 5 - APDREQTRIG"]
    #[inline(always)]
    #[must_use]
    pub fn apdreqtrig(&mut self) -> APDREQTRIG_W<5> {
        APDREQTRIG_W::new(self)
    }
    #[doc = "Bit 6 - DRRDIS"]
    #[inline(always)]
    #[must_use]
    pub fn drrdis(&mut self) -> DRRDIS_W<6> {
        DRRDIS_W::new(self)
    }
    #[doc = "Bits 8:15 - DN"]
    #[inline(always)]
    #[must_use]
    pub fn dn(&mut self) -> DN_W<8> {
        DN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register controls the PTP Offload Engine operation. This register is available only when the Enable PTP Timestamp Offload feature is selected.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_macpocr](index.html) module"]
pub struct ETH_MACPOCR_SPEC;
impl crate::RegisterSpec for ETH_MACPOCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_macpocr::R](R) reader structure"]
impl crate::Readable for ETH_MACPOCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eth_macpocr::W](W) writer structure"]
impl crate::Writable for ETH_MACPOCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ETH_MACPOCR to value 0"]
impl crate::Resettable for ETH_MACPOCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
