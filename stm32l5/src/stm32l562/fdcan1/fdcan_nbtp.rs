#[doc = "Register `FDCAN_NBTP` reader"]
pub struct R(crate::R<FDCAN_NBTP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDCAN_NBTP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDCAN_NBTP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDCAN_NBTP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FDCAN_NBTP` writer"]
pub struct W(crate::W<FDCAN_NBTP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FDCAN_NBTP_SPEC>;
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
impl From<crate::W<FDCAN_NBTP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FDCAN_NBTP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TSEG2` reader - Nominal Time segment after sample point"]
pub type TSEG2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TSEG2` writer - Nominal Time segment after sample point"]
pub type TSEG2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FDCAN_NBTP_SPEC, u8, u8, 7, O>;
#[doc = "Field `NTSEG1` reader - Nominal Time segment before sample point"]
pub type NTSEG1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NTSEG1` writer - Nominal Time segment before sample point"]
pub type NTSEG1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FDCAN_NBTP_SPEC, u8, u8, 8, O>;
#[doc = "Field `NBRP` reader - Bit Rate Prescaler"]
pub type NBRP_R = crate::FieldReader<u16, u16>;
#[doc = "Field `NBRP` writer - Bit Rate Prescaler"]
pub type NBRP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FDCAN_NBTP_SPEC, u16, u16, 9, O>;
#[doc = "Field `NSJW` reader - NSJW: Nominal (Re)Synchronization Jump Width"]
pub type NSJW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NSJW` writer - NSJW: Nominal (Re)Synchronization Jump Width"]
pub type NSJW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FDCAN_NBTP_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 0:6 - Nominal Time segment after sample point"]
    #[inline(always)]
    pub fn tseg2(&self) -> TSEG2_R {
        TSEG2_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:15 - Nominal Time segment before sample point"]
    #[inline(always)]
    pub fn ntseg1(&self) -> NTSEG1_R {
        NTSEG1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:24 - Bit Rate Prescaler"]
    #[inline(always)]
    pub fn nbrp(&self) -> NBRP_R {
        NBRP_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
    #[doc = "Bits 25:31 - NSJW: Nominal (Re)Synchronization Jump Width"]
    #[inline(always)]
    pub fn nsjw(&self) -> NSJW_R {
        NSJW_R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Nominal Time segment after sample point"]
    #[inline(always)]
    #[must_use]
    pub fn tseg2(&mut self) -> TSEG2_W<0> {
        TSEG2_W::new(self)
    }
    #[doc = "Bits 8:15 - Nominal Time segment before sample point"]
    #[inline(always)]
    #[must_use]
    pub fn ntseg1(&mut self) -> NTSEG1_W<8> {
        NTSEG1_W::new(self)
    }
    #[doc = "Bits 16:24 - Bit Rate Prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn nbrp(&mut self) -> NBRP_W<16> {
        NBRP_W::new(self)
    }
    #[doc = "Bits 25:31 - NSJW: Nominal (Re)Synchronization Jump Width"]
    #[inline(always)]
    #[must_use]
    pub fn nsjw(&mut self) -> NSJW_W<25> {
        NSJW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FDCAN Nominal Bit Timing and Prescaler Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_nbtp](index.html) module"]
pub struct FDCAN_NBTP_SPEC;
impl crate::RegisterSpec for FDCAN_NBTP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fdcan_nbtp::R](R) reader structure"]
impl crate::Readable for FDCAN_NBTP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fdcan_nbtp::W](W) writer structure"]
impl crate::Writable for FDCAN_NBTP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FDCAN_NBTP to value 0x0600_0a03"]
impl crate::Resettable for FDCAN_NBTP_SPEC {
    const RESET_VALUE: Self::Ux = 0x0600_0a03;
}
