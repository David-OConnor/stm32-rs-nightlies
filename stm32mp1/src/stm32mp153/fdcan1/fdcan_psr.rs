#[doc = "Register `FDCAN_PSR` reader"]
pub struct R(crate::R<FDCAN_PSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDCAN_PSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDCAN_PSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDCAN_PSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FDCAN_PSR` writer"]
pub struct W(crate::W<FDCAN_PSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FDCAN_PSR_SPEC>;
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
impl From<crate::W<FDCAN_PSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FDCAN_PSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LEC` reader - LEC"]
pub type LEC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ACT` reader - ACT"]
pub type ACT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EP` reader - EP"]
pub type EP_R = crate::BitReader<bool>;
#[doc = "Field `EW` reader - EW"]
pub type EW_R = crate::BitReader<bool>;
#[doc = "Field `BO` reader - BO"]
pub type BO_R = crate::BitReader<bool>;
#[doc = "Field `DLEC` reader - DLEC"]
pub type DLEC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESI` reader - RESI"]
pub type RESI_R = crate::BitReader<bool>;
#[doc = "Field `RESI` writer - RESI"]
pub type RESI_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_PSR_SPEC, bool, O>;
#[doc = "Field `RBRS` reader - RBRS"]
pub type RBRS_R = crate::BitReader<bool>;
#[doc = "Field `RBRS` writer - RBRS"]
pub type RBRS_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_PSR_SPEC, bool, O>;
#[doc = "Field `REDL` reader - REDL"]
pub type REDL_R = crate::BitReader<bool>;
#[doc = "Field `REDL` writer - REDL"]
pub type REDL_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_PSR_SPEC, bool, O>;
#[doc = "Field `PXE` reader - PXE"]
pub type PXE_R = crate::BitReader<bool>;
#[doc = "Field `PXE` writer - PXE"]
pub type PXE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_PSR_SPEC, bool, O>;
#[doc = "Field `TDCV` reader - TDCV"]
pub type TDCV_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:2 - LEC"]
    #[inline(always)]
    pub fn lec(&self) -> LEC_R {
        LEC_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:4 - ACT"]
    #[inline(always)]
    pub fn act(&self) -> ACT_R {
        ACT_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - EP"]
    #[inline(always)]
    pub fn ep(&self) -> EP_R {
        EP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - EW"]
    #[inline(always)]
    pub fn ew(&self) -> EW_R {
        EW_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - BO"]
    #[inline(always)]
    pub fn bo(&self) -> BO_R {
        BO_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - DLEC"]
    #[inline(always)]
    pub fn dlec(&self) -> DLEC_R {
        DLEC_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - RESI"]
    #[inline(always)]
    pub fn resi(&self) -> RESI_R {
        RESI_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - RBRS"]
    #[inline(always)]
    pub fn rbrs(&self) -> RBRS_R {
        RBRS_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - REDL"]
    #[inline(always)]
    pub fn redl(&self) -> REDL_R {
        REDL_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PXE"]
    #[inline(always)]
    pub fn pxe(&self) -> PXE_R {
        PXE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:22 - TDCV"]
    #[inline(always)]
    pub fn tdcv(&self) -> TDCV_R {
        TDCV_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 11 - RESI"]
    #[inline(always)]
    #[must_use]
    pub fn resi(&mut self) -> RESI_W<11> {
        RESI_W::new(self)
    }
    #[doc = "Bit 12 - RBRS"]
    #[inline(always)]
    #[must_use]
    pub fn rbrs(&mut self) -> RBRS_W<12> {
        RBRS_W::new(self)
    }
    #[doc = "Bit 13 - REDL"]
    #[inline(always)]
    #[must_use]
    pub fn redl(&mut self) -> REDL_W<13> {
        REDL_W::new(self)
    }
    #[doc = "Bit 14 - PXE"]
    #[inline(always)]
    #[must_use]
    pub fn pxe(&mut self) -> PXE_W<14> {
        PXE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FDCAN protocol status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_psr](index.html) module"]
pub struct FDCAN_PSR_SPEC;
impl crate::RegisterSpec for FDCAN_PSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fdcan_psr::R](R) reader structure"]
impl crate::Readable for FDCAN_PSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fdcan_psr::W](W) writer structure"]
impl crate::Writable for FDCAN_PSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FDCAN_PSR to value 0x0707"]
impl crate::Resettable for FDCAN_PSR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0707;
}
