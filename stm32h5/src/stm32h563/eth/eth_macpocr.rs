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
#[doc = "Field `PTOEN` reader - PTP Offload Enable When this bit is set, the PTP Offload feature is enabled."]
pub type PTOEN_R = crate::BitReader<bool>;
#[doc = "Field `PTOEN` writer - PTP Offload Enable When this bit is set, the PTP Offload feature is enabled."]
pub type PTOEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACPOCR_SPEC, bool, O>;
#[doc = "Field `ASYNCEN` reader - Automatic PTP SYNC message Enable When this bit is set, PTP SYNC message is generated periodically based on interval programmed or trigger from application, when the MAC is programmed to be in Clock Master mode."]
pub type ASYNCEN_R = crate::BitReader<bool>;
#[doc = "Field `ASYNCEN` writer - Automatic PTP SYNC message Enable When this bit is set, PTP SYNC message is generated periodically based on interval programmed or trigger from application, when the MAC is programmed to be in Clock Master mode."]
pub type ASYNCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACPOCR_SPEC, bool, O>;
#[doc = "Field `APDREQEN` reader - Automatic PTP Pdelay_Req message Enable When this bit is set, PTP Pdelay_Req message is generated periodically based on interval programmed or trigger from application, when the MAC is programmed to be in Peer-to-Peer Transparent mode."]
pub type APDREQEN_R = crate::BitReader<bool>;
#[doc = "Field `APDREQEN` writer - Automatic PTP Pdelay_Req message Enable When this bit is set, PTP Pdelay_Req message is generated periodically based on interval programmed or trigger from application, when the MAC is programmed to be in Peer-to-Peer Transparent mode."]
pub type APDREQEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACPOCR_SPEC, bool, O>;
#[doc = "Field `ASYNCTRIG` reader - Automatic PTP SYNC message Trigger When this bit is set, one PTP SYNC message is transmitted. This bit is automatically cleared after the PTP SYNC message is transmitted. The application should set the ASYNCEN bit for this operation."]
pub type ASYNCTRIG_R = crate::BitReader<bool>;
#[doc = "Field `ASYNCTRIG` writer - Automatic PTP SYNC message Trigger When this bit is set, one PTP SYNC message is transmitted. This bit is automatically cleared after the PTP SYNC message is transmitted. The application should set the ASYNCEN bit for this operation."]
pub type ASYNCTRIG_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACPOCR_SPEC, bool, O>;
#[doc = "Field `APDREQTRIG` reader - Automatic PTP Pdelay_Req message Trigger When this bit is set, one PTP Pdelay_Req message is transmitted. This bit is automatically cleared after the PTP Pdelay_Req message is transmitted. The application should set the APDREQEN bit for this operation."]
pub type APDREQTRIG_R = crate::BitReader<bool>;
#[doc = "Field `APDREQTRIG` writer - Automatic PTP Pdelay_Req message Trigger When this bit is set, one PTP Pdelay_Req message is transmitted. This bit is automatically cleared after the PTP Pdelay_Req message is transmitted. The application should set the APDREQEN bit for this operation."]
pub type APDREQTRIG_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACPOCR_SPEC, bool, O>;
#[doc = "Field `DRRDIS` reader - Disable PTO Delay Request/Response response generation When this bit is set, the Delay Request and Delay response are not generated for received SYNC and Delay request packet respectively, as required by the programmed mode."]
pub type DRRDIS_R = crate::BitReader<bool>;
#[doc = "Field `DRRDIS` writer - Disable PTO Delay Request/Response response generation When this bit is set, the Delay Request and Delay response are not generated for received SYNC and Delay request packet respectively, as required by the programmed mode."]
pub type DRRDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACPOCR_SPEC, bool, O>;
#[doc = "Field `DN` reader - Domain Number This field indicates the domain Number in which the PTP node is operating."]
pub type DN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DN` writer - Domain Number This field indicates the domain Number in which the PTP node is operating."]
pub type DN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ETH_MACPOCR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bit 0 - PTP Offload Enable When this bit is set, the PTP Offload feature is enabled."]
    #[inline(always)]
    pub fn ptoen(&self) -> PTOEN_R {
        PTOEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Automatic PTP SYNC message Enable When this bit is set, PTP SYNC message is generated periodically based on interval programmed or trigger from application, when the MAC is programmed to be in Clock Master mode."]
    #[inline(always)]
    pub fn asyncen(&self) -> ASYNCEN_R {
        ASYNCEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Automatic PTP Pdelay_Req message Enable When this bit is set, PTP Pdelay_Req message is generated periodically based on interval programmed or trigger from application, when the MAC is programmed to be in Peer-to-Peer Transparent mode."]
    #[inline(always)]
    pub fn apdreqen(&self) -> APDREQEN_R {
        APDREQEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Automatic PTP SYNC message Trigger When this bit is set, one PTP SYNC message is transmitted. This bit is automatically cleared after the PTP SYNC message is transmitted. The application should set the ASYNCEN bit for this operation."]
    #[inline(always)]
    pub fn asynctrig(&self) -> ASYNCTRIG_R {
        ASYNCTRIG_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Automatic PTP Pdelay_Req message Trigger When this bit is set, one PTP Pdelay_Req message is transmitted. This bit is automatically cleared after the PTP Pdelay_Req message is transmitted. The application should set the APDREQEN bit for this operation."]
    #[inline(always)]
    pub fn apdreqtrig(&self) -> APDREQTRIG_R {
        APDREQTRIG_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Disable PTO Delay Request/Response response generation When this bit is set, the Delay Request and Delay response are not generated for received SYNC and Delay request packet respectively, as required by the programmed mode."]
    #[inline(always)]
    pub fn drrdis(&self) -> DRRDIS_R {
        DRRDIS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Domain Number This field indicates the domain Number in which the PTP node is operating."]
    #[inline(always)]
    pub fn dn(&self) -> DN_R {
        DN_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - PTP Offload Enable When this bit is set, the PTP Offload feature is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn ptoen(&mut self) -> PTOEN_W<0> {
        PTOEN_W::new(self)
    }
    #[doc = "Bit 1 - Automatic PTP SYNC message Enable When this bit is set, PTP SYNC message is generated periodically based on interval programmed or trigger from application, when the MAC is programmed to be in Clock Master mode."]
    #[inline(always)]
    #[must_use]
    pub fn asyncen(&mut self) -> ASYNCEN_W<1> {
        ASYNCEN_W::new(self)
    }
    #[doc = "Bit 2 - Automatic PTP Pdelay_Req message Enable When this bit is set, PTP Pdelay_Req message is generated periodically based on interval programmed or trigger from application, when the MAC is programmed to be in Peer-to-Peer Transparent mode."]
    #[inline(always)]
    #[must_use]
    pub fn apdreqen(&mut self) -> APDREQEN_W<2> {
        APDREQEN_W::new(self)
    }
    #[doc = "Bit 4 - Automatic PTP SYNC message Trigger When this bit is set, one PTP SYNC message is transmitted. This bit is automatically cleared after the PTP SYNC message is transmitted. The application should set the ASYNCEN bit for this operation."]
    #[inline(always)]
    #[must_use]
    pub fn asynctrig(&mut self) -> ASYNCTRIG_W<4> {
        ASYNCTRIG_W::new(self)
    }
    #[doc = "Bit 5 - Automatic PTP Pdelay_Req message Trigger When this bit is set, one PTP Pdelay_Req message is transmitted. This bit is automatically cleared after the PTP Pdelay_Req message is transmitted. The application should set the APDREQEN bit for this operation."]
    #[inline(always)]
    #[must_use]
    pub fn apdreqtrig(&mut self) -> APDREQTRIG_W<5> {
        APDREQTRIG_W::new(self)
    }
    #[doc = "Bit 6 - Disable PTO Delay Request/Response response generation When this bit is set, the Delay Request and Delay response are not generated for received SYNC and Delay request packet respectively, as required by the programmed mode."]
    #[inline(always)]
    #[must_use]
    pub fn drrdis(&mut self) -> DRRDIS_W<6> {
        DRRDIS_W::new(self)
    }
    #[doc = "Bits 8:15 - Domain Number This field indicates the domain Number in which the PTP node is operating."]
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
#[doc = "PTP Offload control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_macpocr](index.html) module"]
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
