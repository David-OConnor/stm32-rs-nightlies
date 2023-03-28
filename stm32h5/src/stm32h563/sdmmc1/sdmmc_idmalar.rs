#[doc = "Register `SDMMC_IDMALAR` reader"]
pub struct R(crate::R<SDMMC_IDMALAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDMMC_IDMALAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDMMC_IDMALAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDMMC_IDMALAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SDMMC_IDMALAR` writer"]
pub struct W(crate::W<SDMMC_IDMALAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SDMMC_IDMALAR_SPEC>;
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
impl From<crate::W<SDMMC_IDMALAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SDMMC_IDMALAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IDMALA` reader - Word aligned linked list item address offset Linked list item offset pointer to the base of the next linked list item structure. Linked list item base address is IDMABA + IDMALA. These bits can only be written by firmware when DPSM is inactive (DPSMACT = 0)."]
pub type IDMALA_R = crate::FieldReader<u16, u16>;
#[doc = "Field `IDMALA` writer - Word aligned linked list item address offset Linked list item offset pointer to the base of the next linked list item structure. Linked list item base address is IDMABA + IDMALA. These bits can only be written by firmware when DPSM is inactive (DPSMACT = 0)."]
pub type IDMALA_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SDMMC_IDMALAR_SPEC, u16, u16, 14, O>;
#[doc = "Field `ABR` reader - Acknowledge linked list buffer ready This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0). This bit is not taken into account when starting the first linked list buffer from the software programmed register information. ABR is only taken into account on subsequent loaded linked list items."]
pub type ABR_R = crate::BitReader<bool>;
#[doc = "Field `ABR` writer - Acknowledge linked list buffer ready This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0). This bit is not taken into account when starting the first linked list buffer from the software programmed register information. ABR is only taken into account on subsequent loaded linked list items."]
pub type ABR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SDMMC_IDMALAR_SPEC, bool, O>;
#[doc = "Field `ULS` reader - Update SDMMC_IDMABSIZE from the next linked list when in linked list mode (SDMMC_IDMACTRLR.IDMABMODE select linked list mode and ULA = 1) This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0)."]
pub type ULS_R = crate::BitReader<bool>;
#[doc = "Field `ULS` writer - Update SDMMC_IDMABSIZE from the next linked list when in linked list mode (SDMMC_IDMACTRLR.IDMABMODE select linked list mode and ULA = 1) This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0)."]
pub type ULS_W<'a, const O: u8> = crate::BitWriter<'a, u32, SDMMC_IDMALAR_SPEC, bool, O>;
#[doc = "Field `ULA` reader - Update SDMMC_IDMALAR from linked list when in linked list mode (SDMMC_IDMACTRLR.IDMABMODE select linked list mode) This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0)."]
pub type ULA_R = crate::BitReader<bool>;
#[doc = "Field `ULA` writer - Update SDMMC_IDMALAR from linked list when in linked list mode (SDMMC_IDMACTRLR.IDMABMODE select linked list mode) This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0)."]
pub type ULA_W<'a, const O: u8> = crate::BitWriter<'a, u32, SDMMC_IDMALAR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 2:15 - Word aligned linked list item address offset Linked list item offset pointer to the base of the next linked list item structure. Linked list item base address is IDMABA + IDMALA. These bits can only be written by firmware when DPSM is inactive (DPSMACT = 0)."]
    #[inline(always)]
    pub fn idmala(&self) -> IDMALA_R {
        IDMALA_R::new(((self.bits >> 2) & 0x3fff) as u16)
    }
    #[doc = "Bit 29 - Acknowledge linked list buffer ready This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0). This bit is not taken into account when starting the first linked list buffer from the software programmed register information. ABR is only taken into account on subsequent loaded linked list items."]
    #[inline(always)]
    pub fn abr(&self) -> ABR_R {
        ABR_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Update SDMMC_IDMABSIZE from the next linked list when in linked list mode (SDMMC_IDMACTRLR.IDMABMODE select linked list mode and ULA = 1) This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0)."]
    #[inline(always)]
    pub fn uls(&self) -> ULS_R {
        ULS_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Update SDMMC_IDMALAR from linked list when in linked list mode (SDMMC_IDMACTRLR.IDMABMODE select linked list mode) This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0)."]
    #[inline(always)]
    pub fn ula(&self) -> ULA_R {
        ULA_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 2:15 - Word aligned linked list item address offset Linked list item offset pointer to the base of the next linked list item structure. Linked list item base address is IDMABA + IDMALA. These bits can only be written by firmware when DPSM is inactive (DPSMACT = 0)."]
    #[inline(always)]
    #[must_use]
    pub fn idmala(&mut self) -> IDMALA_W<2> {
        IDMALA_W::new(self)
    }
    #[doc = "Bit 29 - Acknowledge linked list buffer ready This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0). This bit is not taken into account when starting the first linked list buffer from the software programmed register information. ABR is only taken into account on subsequent loaded linked list items."]
    #[inline(always)]
    #[must_use]
    pub fn abr(&mut self) -> ABR_W<29> {
        ABR_W::new(self)
    }
    #[doc = "Bit 30 - Update SDMMC_IDMABSIZE from the next linked list when in linked list mode (SDMMC_IDMACTRLR.IDMABMODE select linked list mode and ULA = 1) This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0)."]
    #[inline(always)]
    #[must_use]
    pub fn uls(&mut self) -> ULS_W<30> {
        ULS_W::new(self)
    }
    #[doc = "Bit 31 - Update SDMMC_IDMALAR from linked list when in linked list mode (SDMMC_IDMACTRLR.IDMABMODE select linked list mode) This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0)."]
    #[inline(always)]
    #[must_use]
    pub fn ula(&mut self) -> ULA_W<31> {
        ULA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SDMMC_IDMALAR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdmmc_idmalar](index.html) module"]
pub struct SDMMC_IDMALAR_SPEC;
impl crate::RegisterSpec for SDMMC_IDMALAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sdmmc_idmalar::R](R) reader structure"]
impl crate::Readable for SDMMC_IDMALAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sdmmc_idmalar::W](W) writer structure"]
impl crate::Writable for SDMMC_IDMALAR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SDMMC_IDMALAR to value 0"]
impl crate::Resettable for SDMMC_IDMALAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
