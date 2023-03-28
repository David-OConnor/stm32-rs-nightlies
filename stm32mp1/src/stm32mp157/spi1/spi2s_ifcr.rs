#[doc = "Register `SPI2S_IFCR` writer"]
pub struct W(crate::W<SPI2S_IFCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI2S_IFCR_SPEC>;
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
impl From<crate::W<SPI2S_IFCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI2S_IFCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EOTC` writer - EOTC"]
pub type EOTC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI2S_IFCR_SPEC, bool, O>;
#[doc = "Field `TXTFC` writer - TXTFC"]
pub type TXTFC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI2S_IFCR_SPEC, bool, O>;
#[doc = "Field `UDRC` writer - UDRC"]
pub type UDRC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI2S_IFCR_SPEC, bool, O>;
#[doc = "Field `OVRC` writer - OVRC"]
pub type OVRC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI2S_IFCR_SPEC, bool, O>;
#[doc = "Field `CRCEC` writer - CRCEC"]
pub type CRCEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI2S_IFCR_SPEC, bool, O>;
#[doc = "Field `TIFREC` writer - TIFREC"]
pub type TIFREC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI2S_IFCR_SPEC, bool, O>;
#[doc = "Field `MODFC` writer - MODFC"]
pub type MODFC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI2S_IFCR_SPEC, bool, O>;
#[doc = "Field `TSERFC` writer - TSERFC"]
pub type TSERFC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI2S_IFCR_SPEC, bool, O>;
#[doc = "Field `SUSPC` writer - SUSPC"]
pub type SUSPC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI2S_IFCR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 3 - EOTC"]
    #[inline(always)]
    #[must_use]
    pub fn eotc(&mut self) -> EOTC_W<3> {
        EOTC_W::new(self)
    }
    #[doc = "Bit 4 - TXTFC"]
    #[inline(always)]
    #[must_use]
    pub fn txtfc(&mut self) -> TXTFC_W<4> {
        TXTFC_W::new(self)
    }
    #[doc = "Bit 5 - UDRC"]
    #[inline(always)]
    #[must_use]
    pub fn udrc(&mut self) -> UDRC_W<5> {
        UDRC_W::new(self)
    }
    #[doc = "Bit 6 - OVRC"]
    #[inline(always)]
    #[must_use]
    pub fn ovrc(&mut self) -> OVRC_W<6> {
        OVRC_W::new(self)
    }
    #[doc = "Bit 7 - CRCEC"]
    #[inline(always)]
    #[must_use]
    pub fn crcec(&mut self) -> CRCEC_W<7> {
        CRCEC_W::new(self)
    }
    #[doc = "Bit 8 - TIFREC"]
    #[inline(always)]
    #[must_use]
    pub fn tifrec(&mut self) -> TIFREC_W<8> {
        TIFREC_W::new(self)
    }
    #[doc = "Bit 9 - MODFC"]
    #[inline(always)]
    #[must_use]
    pub fn modfc(&mut self) -> MODFC_W<9> {
        MODFC_W::new(self)
    }
    #[doc = "Bit 10 - TSERFC"]
    #[inline(always)]
    #[must_use]
    pub fn tserfc(&mut self) -> TSERFC_W<10> {
        TSERFC_W::new(self)
    }
    #[doc = "Bit 11 - SUSPC"]
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
#[doc = "SPI/I2S interrupt/status flags clear register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi2s_ifcr](index.html) module"]
pub struct SPI2S_IFCR_SPEC;
impl crate::RegisterSpec for SPI2S_IFCR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [spi2s_ifcr::W](W) writer structure"]
impl crate::Writable for SPI2S_IFCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPI2S_IFCR to value 0"]
impl crate::Resettable for SPI2S_IFCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
