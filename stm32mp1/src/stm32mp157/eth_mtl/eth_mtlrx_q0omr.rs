#[doc = "Register `ETH_MTLRxQ0OMR` reader"]
pub struct R(crate::R<ETH_MTLRX_Q0OMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_MTLRX_Q0OMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_MTLRX_Q0OMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_MTLRX_Q0OMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETH_MTLRxQ0OMR` writer"]
pub struct W(crate::W<ETH_MTLRX_Q0OMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETH_MTLRX_Q0OMR_SPEC>;
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
impl From<crate::W<ETH_MTLRX_Q0OMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETH_MTLRX_Q0OMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RTC` reader - RTC"]
pub type RTC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RTC` writer - RTC"]
pub type RTC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ETH_MTLRX_Q0OMR_SPEC, u8, u8, 2, O>;
#[doc = "Field `FUP` reader - FUP"]
pub type FUP_R = crate::BitReader<bool>;
#[doc = "Field `FUP` writer - FUP"]
pub type FUP_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MTLRX_Q0OMR_SPEC, bool, O>;
#[doc = "Field `FEP` reader - FEP"]
pub type FEP_R = crate::BitReader<bool>;
#[doc = "Field `FEP` writer - FEP"]
pub type FEP_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MTLRX_Q0OMR_SPEC, bool, O>;
#[doc = "Field `RSF` reader - RSF"]
pub type RSF_R = crate::BitReader<bool>;
#[doc = "Field `RSF` writer - RSF"]
pub type RSF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MTLRX_Q0OMR_SPEC, bool, O>;
#[doc = "Field `DIS_TCP_EF` reader - DIS_TCP_EF"]
pub type DIS_TCP_EF_R = crate::BitReader<bool>;
#[doc = "Field `DIS_TCP_EF` writer - DIS_TCP_EF"]
pub type DIS_TCP_EF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MTLRX_Q0OMR_SPEC, bool, O>;
#[doc = "Field `EHFC` reader - EHFC"]
pub type EHFC_R = crate::BitReader<bool>;
#[doc = "Field `EHFC` writer - EHFC"]
pub type EHFC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MTLRX_Q0OMR_SPEC, bool, O>;
#[doc = "Field `RFA` reader - RFA"]
pub type RFA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RFA` writer - RFA"]
pub type RFA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ETH_MTLRX_Q0OMR_SPEC, u8, u8, 3, O>;
#[doc = "Field `RFD` reader - RFD"]
pub type RFD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RFD` writer - RFD"]
pub type RFD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ETH_MTLRX_Q0OMR_SPEC, u8, u8, 3, O>;
#[doc = "Field `RQS` reader - RQS"]
pub type RQS_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:1 - RTC"]
    #[inline(always)]
    pub fn rtc(&self) -> RTC_R {
        RTC_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 3 - FUP"]
    #[inline(always)]
    pub fn fup(&self) -> FUP_R {
        FUP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - FEP"]
    #[inline(always)]
    pub fn fep(&self) -> FEP_R {
        FEP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RSF"]
    #[inline(always)]
    pub fn rsf(&self) -> RSF_R {
        RSF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - DIS_TCP_EF"]
    #[inline(always)]
    pub fn dis_tcp_ef(&self) -> DIS_TCP_EF_R {
        DIS_TCP_EF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - EHFC"]
    #[inline(always)]
    pub fn ehfc(&self) -> EHFC_R {
        EHFC_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - RFA"]
    #[inline(always)]
    pub fn rfa(&self) -> RFA_R {
        RFA_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 14:16 - RFD"]
    #[inline(always)]
    pub fn rfd(&self) -> RFD_R {
        RFD_R::new(((self.bits >> 14) & 7) as u8)
    }
    #[doc = "Bits 20:23 - RQS"]
    #[inline(always)]
    pub fn rqs(&self) -> RQS_R {
        RQS_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - RTC"]
    #[inline(always)]
    #[must_use]
    pub fn rtc(&mut self) -> RTC_W<0> {
        RTC_W::new(self)
    }
    #[doc = "Bit 3 - FUP"]
    #[inline(always)]
    #[must_use]
    pub fn fup(&mut self) -> FUP_W<3> {
        FUP_W::new(self)
    }
    #[doc = "Bit 4 - FEP"]
    #[inline(always)]
    #[must_use]
    pub fn fep(&mut self) -> FEP_W<4> {
        FEP_W::new(self)
    }
    #[doc = "Bit 5 - RSF"]
    #[inline(always)]
    #[must_use]
    pub fn rsf(&mut self) -> RSF_W<5> {
        RSF_W::new(self)
    }
    #[doc = "Bit 6 - DIS_TCP_EF"]
    #[inline(always)]
    #[must_use]
    pub fn dis_tcp_ef(&mut self) -> DIS_TCP_EF_W<6> {
        DIS_TCP_EF_W::new(self)
    }
    #[doc = "Bit 7 - EHFC"]
    #[inline(always)]
    #[must_use]
    pub fn ehfc(&mut self) -> EHFC_W<7> {
        EHFC_W::new(self)
    }
    #[doc = "Bits 8:10 - RFA"]
    #[inline(always)]
    #[must_use]
    pub fn rfa(&mut self) -> RFA_W<8> {
        RFA_W::new(self)
    }
    #[doc = "Bits 14:16 - RFD"]
    #[inline(always)]
    #[must_use]
    pub fn rfd(&mut self) -> RFD_W<14> {
        RFD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Rx queue 0 operating mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_mtlrx_q0omr](index.html) module"]
pub struct ETH_MTLRX_Q0OMR_SPEC;
impl crate::RegisterSpec for ETH_MTLRX_Q0OMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_mtlrx_q0omr::R](R) reader structure"]
impl crate::Readable for ETH_MTLRX_Q0OMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eth_mtlrx_q0omr::W](W) writer structure"]
impl crate::Writable for ETH_MTLRX_Q0OMR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ETH_MTLRxQ0OMR to value 0x0070_0000"]
impl crate::Resettable for ETH_MTLRX_Q0OMR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0070_0000;
}
