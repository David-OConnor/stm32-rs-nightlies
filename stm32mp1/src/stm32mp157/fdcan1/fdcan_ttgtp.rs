#[doc = "Register `FDCAN_TTGTP` reader"]
pub struct R(crate::R<FDCAN_TTGTP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDCAN_TTGTP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDCAN_TTGTP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDCAN_TTGTP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FDCAN_TTGTP` writer"]
pub struct W(crate::W<FDCAN_TTGTP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FDCAN_TTGTP_SPEC>;
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
impl From<crate::W<FDCAN_TTGTP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FDCAN_TTGTP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TP` reader - TP"]
pub type TP_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TP` writer - TP"]
pub type TP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FDCAN_TTGTP_SPEC, u16, u16, 16, O>;
#[doc = "Field `CTP` reader - CTP"]
pub type CTP_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CTP` writer - CTP"]
pub type CTP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FDCAN_TTGTP_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - TP"]
    #[inline(always)]
    pub fn tp(&self) -> TP_R {
        TP_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - CTP"]
    #[inline(always)]
    pub fn ctp(&self) -> CTP_R {
        CTP_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - TP"]
    #[inline(always)]
    #[must_use]
    pub fn tp(&mut self) -> TP_W<0> {
        TP_W::new(self)
    }
    #[doc = "Bits 16:31 - CTP"]
    #[inline(always)]
    #[must_use]
    pub fn ctp(&mut self) -> CTP_W<16> {
        CTP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "If TTOST.WGDT is set, the next reference message will be transmitted with the Master_Ref_Mark modified by the preset value and with Disc_Bit = 1, presetting the global time in all nodes simultaneously. TP is reset to 0x0000 each time a reference message with Disc_Bit = 1 becomes valid or if the node is not the current time master. TP is locked while FDCAN_TTOST.WGTD = 1 after setting FDCAN_TTOCN.SGT until the reference message with Disc_Bit = 1 becomes valid or until the node is no longer the current time master.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_ttgtp](index.html) module"]
pub struct FDCAN_TTGTP_SPEC;
impl crate::RegisterSpec for FDCAN_TTGTP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fdcan_ttgtp::R](R) reader structure"]
impl crate::Readable for FDCAN_TTGTP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fdcan_ttgtp::W](W) writer structure"]
impl crate::Writable for FDCAN_TTGTP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FDCAN_TTGTP to value 0"]
impl crate::Resettable for FDCAN_TTGTP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
