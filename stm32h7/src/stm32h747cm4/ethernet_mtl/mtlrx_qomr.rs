#[doc = "Register `MTLRxQOMR` reader"]
pub struct R(crate::R<MTLRX_QOMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MTLRX_QOMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MTLRX_QOMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MTLRX_QOMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MTLRxQOMR` writer"]
pub struct W(crate::W<MTLRX_QOMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MTLRX_QOMR_SPEC>;
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
impl From<crate::W<MTLRX_QOMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MTLRX_QOMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RTC` reader - Receive Queue Threshold Control"]
pub type RTC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RTC` writer - Receive Queue Threshold Control"]
pub type RTC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MTLRX_QOMR_SPEC, u8, u8, 2, O>;
#[doc = "Field `FUP` reader - Forward Undersized Good Packets"]
pub type FUP_R = crate::BitReader<bool>;
#[doc = "Field `FUP` writer - Forward Undersized Good Packets"]
pub type FUP_W<'a, const O: u8> = crate::BitWriter<'a, u32, MTLRX_QOMR_SPEC, bool, O>;
#[doc = "Field `FEP` reader - Forward Error Packets"]
pub type FEP_R = crate::BitReader<bool>;
#[doc = "Field `FEP` writer - Forward Error Packets"]
pub type FEP_W<'a, const O: u8> = crate::BitWriter<'a, u32, MTLRX_QOMR_SPEC, bool, O>;
#[doc = "Field `RSF` reader - Receive Queue Store and Forward"]
pub type RSF_R = crate::BitReader<bool>;
#[doc = "Field `RSF` writer - Receive Queue Store and Forward"]
pub type RSF_W<'a, const O: u8> = crate::BitWriter<'a, u32, MTLRX_QOMR_SPEC, bool, O>;
#[doc = "Field `DIS_TCP_EF` reader - Disable Dropping of TCP"]
pub type DIS_TCP_EF_R = crate::BitReader<bool>;
#[doc = "Field `DIS_TCP_EF` writer - Disable Dropping of TCP"]
pub type DIS_TCP_EF_W<'a, const O: u8> = crate::BitWriter<'a, u32, MTLRX_QOMR_SPEC, bool, O>;
#[doc = "Field `EHFC` reader - Enable Hardware Flow Control"]
pub type EHFC_R = crate::BitReader<bool>;
#[doc = "Field `EHFC` writer - Enable Hardware Flow Control"]
pub type EHFC_W<'a, const O: u8> = crate::BitWriter<'a, u32, MTLRX_QOMR_SPEC, bool, O>;
#[doc = "Field `RFA` reader - Threshold for Activating Flow Control"]
pub type RFA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RFA` writer - Threshold for Activating Flow Control"]
pub type RFA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MTLRX_QOMR_SPEC, u8, u8, 3, O>;
#[doc = "Field `RFD` reader - Threshold for Deactivating Flow Control"]
pub type RFD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RFD` writer - Threshold for Deactivating Flow Control"]
pub type RFD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MTLRX_QOMR_SPEC, u8, u8, 3, O>;
#[doc = "Field `RQS` reader - Receive Queue Size"]
pub type RQS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RQS` writer - Receive Queue Size"]
pub type RQS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MTLRX_QOMR_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:1 - Receive Queue Threshold Control"]
    #[inline(always)]
    pub fn rtc(&self) -> RTC_R {
        RTC_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 3 - Forward Undersized Good Packets"]
    #[inline(always)]
    pub fn fup(&self) -> FUP_R {
        FUP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Forward Error Packets"]
    #[inline(always)]
    pub fn fep(&self) -> FEP_R {
        FEP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Receive Queue Store and Forward"]
    #[inline(always)]
    pub fn rsf(&self) -> RSF_R {
        RSF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Disable Dropping of TCP"]
    #[inline(always)]
    pub fn dis_tcp_ef(&self) -> DIS_TCP_EF_R {
        DIS_TCP_EF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable Hardware Flow Control"]
    #[inline(always)]
    pub fn ehfc(&self) -> EHFC_R {
        EHFC_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Threshold for Activating Flow Control"]
    #[inline(always)]
    pub fn rfa(&self) -> RFA_R {
        RFA_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 14:16 - Threshold for Deactivating Flow Control"]
    #[inline(always)]
    pub fn rfd(&self) -> RFD_R {
        RFD_R::new(((self.bits >> 14) & 7) as u8)
    }
    #[doc = "Bits 20:22 - Receive Queue Size"]
    #[inline(always)]
    pub fn rqs(&self) -> RQS_R {
        RQS_R::new(((self.bits >> 20) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Receive Queue Threshold Control"]
    #[inline(always)]
    #[must_use]
    pub fn rtc(&mut self) -> RTC_W<0> {
        RTC_W::new(self)
    }
    #[doc = "Bit 3 - Forward Undersized Good Packets"]
    #[inline(always)]
    #[must_use]
    pub fn fup(&mut self) -> FUP_W<3> {
        FUP_W::new(self)
    }
    #[doc = "Bit 4 - Forward Error Packets"]
    #[inline(always)]
    #[must_use]
    pub fn fep(&mut self) -> FEP_W<4> {
        FEP_W::new(self)
    }
    #[doc = "Bit 5 - Receive Queue Store and Forward"]
    #[inline(always)]
    #[must_use]
    pub fn rsf(&mut self) -> RSF_W<5> {
        RSF_W::new(self)
    }
    #[doc = "Bit 6 - Disable Dropping of TCP"]
    #[inline(always)]
    #[must_use]
    pub fn dis_tcp_ef(&mut self) -> DIS_TCP_EF_W<6> {
        DIS_TCP_EF_W::new(self)
    }
    #[doc = "Bit 7 - Enable Hardware Flow Control"]
    #[inline(always)]
    #[must_use]
    pub fn ehfc(&mut self) -> EHFC_W<7> {
        EHFC_W::new(self)
    }
    #[doc = "Bits 8:10 - Threshold for Activating Flow Control"]
    #[inline(always)]
    #[must_use]
    pub fn rfa(&mut self) -> RFA_W<8> {
        RFA_W::new(self)
    }
    #[doc = "Bits 14:16 - Threshold for Deactivating Flow Control"]
    #[inline(always)]
    #[must_use]
    pub fn rfd(&mut self) -> RFD_W<14> {
        RFD_W::new(self)
    }
    #[doc = "Bits 20:22 - Receive Queue Size"]
    #[inline(always)]
    #[must_use]
    pub fn rqs(&mut self) -> RQS_W<20> {
        RQS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Rx queue operating mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mtlrx_qomr](index.html) module"]
pub struct MTLRX_QOMR_SPEC;
impl crate::RegisterSpec for MTLRX_QOMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mtlrx_qomr::R](R) reader structure"]
impl crate::Readable for MTLRX_QOMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mtlrx_qomr::W](W) writer structure"]
impl crate::Writable for MTLRX_QOMR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MTLRxQOMR to value 0x0070_0000"]
impl crate::Resettable for MTLRX_QOMR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0070_0000;
}
