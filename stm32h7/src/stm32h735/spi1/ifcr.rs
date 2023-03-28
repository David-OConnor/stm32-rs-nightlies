#[doc = "Register `IFCR` writer"]
pub struct W(crate::W<IFCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IFCR_SPEC>;
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
impl From<crate::W<IFCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IFCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "End Of Transfer flag clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOTCW_AW {
    #[doc = "1: Clear interrupt flag"]
    Clear = 1,
}
impl From<EOTCW_AW> for bool {
    #[inline(always)]
    fn from(variant: EOTCW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOTC` writer - End Of Transfer flag clear"]
pub type EOTC_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFCR_SPEC, EOTCW_AW, O>;
impl<'a, const O: u8> EOTC_W<'a, O> {
    #[doc = "Clear interrupt flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(EOTCW_AW::Clear)
    }
}
#[doc = "Field `TXTFC` writer - Transmission Transfer Filled flag clear"]
pub use EOTC_W as TXTFC_W;
#[doc = "Field `UDRC` writer - Underrun flag clear"]
pub use EOTC_W as UDRC_W;
#[doc = "Field `OVRC` writer - Overrun flag clear"]
pub use EOTC_W as OVRC_W;
#[doc = "Field `CRCEC` writer - CRC Error flag clear"]
pub use EOTC_W as CRCEC_W;
#[doc = "Field `TIFREC` writer - TI frame format error flag clear"]
pub use EOTC_W as TIFREC_W;
#[doc = "Field `MODFC` writer - Mode Fault flag clear"]
pub use EOTC_W as MODFC_W;
#[doc = "Field `TSERFC` writer - TSERFC flag clear"]
pub use EOTC_W as TSERFC_W;
#[doc = "Field `SUSPC` writer - SUSPend flag clear"]
pub use EOTC_W as SUSPC_W;
impl W {
    #[doc = "Bit 3 - End Of Transfer flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn eotc(&mut self) -> EOTC_W<3> {
        EOTC_W::new(self)
    }
    #[doc = "Bit 4 - Transmission Transfer Filled flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn txtfc(&mut self) -> TXTFC_W<4> {
        TXTFC_W::new(self)
    }
    #[doc = "Bit 5 - Underrun flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn udrc(&mut self) -> UDRC_W<5> {
        UDRC_W::new(self)
    }
    #[doc = "Bit 6 - Overrun flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn ovrc(&mut self) -> OVRC_W<6> {
        OVRC_W::new(self)
    }
    #[doc = "Bit 7 - CRC Error flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn crcec(&mut self) -> CRCEC_W<7> {
        CRCEC_W::new(self)
    }
    #[doc = "Bit 8 - TI frame format error flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn tifrec(&mut self) -> TIFREC_W<8> {
        TIFREC_W::new(self)
    }
    #[doc = "Bit 9 - Mode Fault flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn modfc(&mut self) -> MODFC_W<9> {
        MODFC_W::new(self)
    }
    #[doc = "Bit 10 - TSERFC flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn tserfc(&mut self) -> TSERFC_W<10> {
        TSERFC_W::new(self)
    }
    #[doc = "Bit 11 - SUSPend flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn suspc(&mut self) -> SUSPC_W<11> {
        SUSPC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt/Status Flags Clear Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ifcr](index.html) module"]
pub struct IFCR_SPEC;
impl crate::RegisterSpec for IFCR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [ifcr::W](W) writer structure"]
impl crate::Writable for IFCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IFCR to value 0"]
impl crate::Resettable for IFCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
