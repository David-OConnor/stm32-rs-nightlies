#[doc = "Register `DCMI_CR` reader"]
pub struct R(crate::R<DCMI_CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCMI_CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCMI_CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCMI_CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCMI_CR` writer"]
pub struct W(crate::W<DCMI_CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCMI_CR_SPEC>;
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
impl From<crate::W<DCMI_CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCMI_CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CAPTURE` reader - Capture enable The camera interface waits for the first start of frame, then a DMA request is generated to transfer the received data into the destination memory. In snapshot mode, the CAPTURE bit is automatically cleared at the end of the first frame received. In continuous grab mode, if the software clears this bit while a capture is ongoing, the bit is effectively cleared after the frame end. Note: The DMA controller and all DCMI configuration registers must be programmed correctly before enabling this bit."]
pub type CAPTURE_R = crate::BitReader<bool>;
#[doc = "Field `CAPTURE` writer - Capture enable The camera interface waits for the first start of frame, then a DMA request is generated to transfer the received data into the destination memory. In snapshot mode, the CAPTURE bit is automatically cleared at the end of the first frame received. In continuous grab mode, if the software clears this bit while a capture is ongoing, the bit is effectively cleared after the frame end. Note: The DMA controller and all DCMI configuration registers must be programmed correctly before enabling this bit."]
pub type CAPTURE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCMI_CR_SPEC, bool, O>;
#[doc = "Field `CM` reader - Capture mode"]
pub type CM_R = crate::BitReader<bool>;
#[doc = "Field `CM` writer - Capture mode"]
pub type CM_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCMI_CR_SPEC, bool, O>;
#[doc = "Field `CROP` reader - Crop feature"]
pub type CROP_R = crate::BitReader<bool>;
#[doc = "Field `CROP` writer - Crop feature"]
pub type CROP_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCMI_CR_SPEC, bool, O>;
#[doc = "Field `JPEG` reader - JPEG format"]
pub type JPEG_R = crate::BitReader<bool>;
#[doc = "Field `JPEG` writer - JPEG format"]
pub type JPEG_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCMI_CR_SPEC, bool, O>;
#[doc = "Field `ESS` reader - Embedded synchronization select Note: Valid only for 8-bit parallel data. HSPOL/VSPOL are ignored when the ESS bit is set. This bit is disabled in JPEG mode."]
pub type ESS_R = crate::BitReader<bool>;
#[doc = "Field `ESS` writer - Embedded synchronization select Note: Valid only for 8-bit parallel data. HSPOL/VSPOL are ignored when the ESS bit is set. This bit is disabled in JPEG mode."]
pub type ESS_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCMI_CR_SPEC, bool, O>;
#[doc = "Field `PCKPOL` reader - Pixel clock polarity This bit configures the capture edge of the pixel clock."]
pub type PCKPOL_R = crate::BitReader<bool>;
#[doc = "Field `PCKPOL` writer - Pixel clock polarity This bit configures the capture edge of the pixel clock."]
pub type PCKPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCMI_CR_SPEC, bool, O>;
#[doc = "Field `HSPOL` reader - Horizontal synchronization polarity This bit indicates the level on the DCMI_HSYNC pin when the data are not valid on the parallel interface."]
pub type HSPOL_R = crate::BitReader<bool>;
#[doc = "Field `HSPOL` writer - Horizontal synchronization polarity This bit indicates the level on the DCMI_HSYNC pin when the data are not valid on the parallel interface."]
pub type HSPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCMI_CR_SPEC, bool, O>;
#[doc = "Field `VSPOL` reader - Vertical synchronization polarity This bit indicates the level on the DCMI_VSYNC pin when the data are not valid on the parallel interface."]
pub type VSPOL_R = crate::BitReader<bool>;
#[doc = "Field `VSPOL` writer - Vertical synchronization polarity This bit indicates the level on the DCMI_VSYNC pin when the data are not valid on the parallel interface."]
pub type VSPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCMI_CR_SPEC, bool, O>;
#[doc = "Field `FCRC` reader - Frame capture rate control These bits define the frequency of frame capture. They are meaningful only in Continuous grab mode. They are ignored in snapshot mode."]
pub type FCRC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FCRC` writer - Frame capture rate control These bits define the frequency of frame capture. They are meaningful only in Continuous grab mode. They are ignored in snapshot mode."]
pub type FCRC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DCMI_CR_SPEC, u8, u8, 2, O>;
#[doc = "Field `EDM` reader - Extended data mode"]
pub type EDM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EDM` writer - Extended data mode"]
pub type EDM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DCMI_CR_SPEC, u8, u8, 2, O>;
#[doc = "Field `ENABLE` reader - DCMI enable Note: The DCMI configuration registers must be programmed correctly before enabling this bit."]
pub type ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE` writer - DCMI enable Note: The DCMI configuration registers must be programmed correctly before enabling this bit."]
pub type ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCMI_CR_SPEC, bool, O>;
#[doc = "Field `BSM` reader - Byte Select mode Note: This mode only works for EDM\\[1:0\\]
= 00. For all other EDM values, this field must be programmed to the reset value."]
pub type BSM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BSM` writer - Byte Select mode Note: This mode only works for EDM\\[1:0\\]
= 00. For all other EDM values, this field must be programmed to the reset value."]
pub type BSM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DCMI_CR_SPEC, u8, u8, 2, O>;
#[doc = "Field `OEBS` reader - Odd/Even Byte Select (Byte Select Start) This bit works in conjunction with BSM field (BSM ≠ 00)."]
pub type OEBS_R = crate::BitReader<bool>;
#[doc = "Field `OEBS` writer - Odd/Even Byte Select (Byte Select Start) This bit works in conjunction with BSM field (BSM ≠ 00)."]
pub type OEBS_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCMI_CR_SPEC, bool, O>;
#[doc = "Field `LSM` reader - Line Select mode"]
pub type LSM_R = crate::BitReader<bool>;
#[doc = "Field `LSM` writer - Line Select mode"]
pub type LSM_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCMI_CR_SPEC, bool, O>;
#[doc = "Field `OELS` reader - Odd/Even Line Select (Line Select Start) This bit works in conjunction with the LSM field (LSM = 1)."]
pub type OELS_R = crate::BitReader<bool>;
#[doc = "Field `OELS` writer - Odd/Even Line Select (Line Select Start) This bit works in conjunction with the LSM field (LSM = 1)."]
pub type OELS_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCMI_CR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Capture enable The camera interface waits for the first start of frame, then a DMA request is generated to transfer the received data into the destination memory. In snapshot mode, the CAPTURE bit is automatically cleared at the end of the first frame received. In continuous grab mode, if the software clears this bit while a capture is ongoing, the bit is effectively cleared after the frame end. Note: The DMA controller and all DCMI configuration registers must be programmed correctly before enabling this bit."]
    #[inline(always)]
    pub fn capture(&self) -> CAPTURE_R {
        CAPTURE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Capture mode"]
    #[inline(always)]
    pub fn cm(&self) -> CM_R {
        CM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Crop feature"]
    #[inline(always)]
    pub fn crop(&self) -> CROP_R {
        CROP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - JPEG format"]
    #[inline(always)]
    pub fn jpeg(&self) -> JPEG_R {
        JPEG_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Embedded synchronization select Note: Valid only for 8-bit parallel data. HSPOL/VSPOL are ignored when the ESS bit is set. This bit is disabled in JPEG mode."]
    #[inline(always)]
    pub fn ess(&self) -> ESS_R {
        ESS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pixel clock polarity This bit configures the capture edge of the pixel clock."]
    #[inline(always)]
    pub fn pckpol(&self) -> PCKPOL_R {
        PCKPOL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Horizontal synchronization polarity This bit indicates the level on the DCMI_HSYNC pin when the data are not valid on the parallel interface."]
    #[inline(always)]
    pub fn hspol(&self) -> HSPOL_R {
        HSPOL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Vertical synchronization polarity This bit indicates the level on the DCMI_VSYNC pin when the data are not valid on the parallel interface."]
    #[inline(always)]
    pub fn vspol(&self) -> VSPOL_R {
        VSPOL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Frame capture rate control These bits define the frequency of frame capture. They are meaningful only in Continuous grab mode. They are ignored in snapshot mode."]
    #[inline(always)]
    pub fn fcrc(&self) -> FCRC_R {
        FCRC_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Extended data mode"]
    #[inline(always)]
    pub fn edm(&self) -> EDM_R {
        EDM_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 14 - DCMI enable Note: The DCMI configuration registers must be programmed correctly before enabling this bit."]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Byte Select mode Note: This mode only works for EDM\\[1:0\\]
= 00. For all other EDM values, this field must be programmed to the reset value."]
    #[inline(always)]
    pub fn bsm(&self) -> BSM_R {
        BSM_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - Odd/Even Byte Select (Byte Select Start) This bit works in conjunction with BSM field (BSM ≠ 00)."]
    #[inline(always)]
    pub fn oebs(&self) -> OEBS_R {
        OEBS_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Line Select mode"]
    #[inline(always)]
    pub fn lsm(&self) -> LSM_R {
        LSM_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Odd/Even Line Select (Line Select Start) This bit works in conjunction with the LSM field (LSM = 1)."]
    #[inline(always)]
    pub fn oels(&self) -> OELS_R {
        OELS_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Capture enable The camera interface waits for the first start of frame, then a DMA request is generated to transfer the received data into the destination memory. In snapshot mode, the CAPTURE bit is automatically cleared at the end of the first frame received. In continuous grab mode, if the software clears this bit while a capture is ongoing, the bit is effectively cleared after the frame end. Note: The DMA controller and all DCMI configuration registers must be programmed correctly before enabling this bit."]
    #[inline(always)]
    #[must_use]
    pub fn capture(&mut self) -> CAPTURE_W<0> {
        CAPTURE_W::new(self)
    }
    #[doc = "Bit 1 - Capture mode"]
    #[inline(always)]
    #[must_use]
    pub fn cm(&mut self) -> CM_W<1> {
        CM_W::new(self)
    }
    #[doc = "Bit 2 - Crop feature"]
    #[inline(always)]
    #[must_use]
    pub fn crop(&mut self) -> CROP_W<2> {
        CROP_W::new(self)
    }
    #[doc = "Bit 3 - JPEG format"]
    #[inline(always)]
    #[must_use]
    pub fn jpeg(&mut self) -> JPEG_W<3> {
        JPEG_W::new(self)
    }
    #[doc = "Bit 4 - Embedded synchronization select Note: Valid only for 8-bit parallel data. HSPOL/VSPOL are ignored when the ESS bit is set. This bit is disabled in JPEG mode."]
    #[inline(always)]
    #[must_use]
    pub fn ess(&mut self) -> ESS_W<4> {
        ESS_W::new(self)
    }
    #[doc = "Bit 5 - Pixel clock polarity This bit configures the capture edge of the pixel clock."]
    #[inline(always)]
    #[must_use]
    pub fn pckpol(&mut self) -> PCKPOL_W<5> {
        PCKPOL_W::new(self)
    }
    #[doc = "Bit 6 - Horizontal synchronization polarity This bit indicates the level on the DCMI_HSYNC pin when the data are not valid on the parallel interface."]
    #[inline(always)]
    #[must_use]
    pub fn hspol(&mut self) -> HSPOL_W<6> {
        HSPOL_W::new(self)
    }
    #[doc = "Bit 7 - Vertical synchronization polarity This bit indicates the level on the DCMI_VSYNC pin when the data are not valid on the parallel interface."]
    #[inline(always)]
    #[must_use]
    pub fn vspol(&mut self) -> VSPOL_W<7> {
        VSPOL_W::new(self)
    }
    #[doc = "Bits 8:9 - Frame capture rate control These bits define the frequency of frame capture. They are meaningful only in Continuous grab mode. They are ignored in snapshot mode."]
    #[inline(always)]
    #[must_use]
    pub fn fcrc(&mut self) -> FCRC_W<8> {
        FCRC_W::new(self)
    }
    #[doc = "Bits 10:11 - Extended data mode"]
    #[inline(always)]
    #[must_use]
    pub fn edm(&mut self) -> EDM_W<10> {
        EDM_W::new(self)
    }
    #[doc = "Bit 14 - DCMI enable Note: The DCMI configuration registers must be programmed correctly before enabling this bit."]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<14> {
        ENABLE_W::new(self)
    }
    #[doc = "Bits 16:17 - Byte Select mode Note: This mode only works for EDM\\[1:0\\]
= 00. For all other EDM values, this field must be programmed to the reset value."]
    #[inline(always)]
    #[must_use]
    pub fn bsm(&mut self) -> BSM_W<16> {
        BSM_W::new(self)
    }
    #[doc = "Bit 18 - Odd/Even Byte Select (Byte Select Start) This bit works in conjunction with BSM field (BSM ≠ 00)."]
    #[inline(always)]
    #[must_use]
    pub fn oebs(&mut self) -> OEBS_W<18> {
        OEBS_W::new(self)
    }
    #[doc = "Bit 19 - Line Select mode"]
    #[inline(always)]
    #[must_use]
    pub fn lsm(&mut self) -> LSM_W<19> {
        LSM_W::new(self)
    }
    #[doc = "Bit 20 - Odd/Even Line Select (Line Select Start) This bit works in conjunction with the LSM field (LSM = 1)."]
    #[inline(always)]
    #[must_use]
    pub fn oels(&mut self) -> OELS_W<20> {
        OELS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DCMI control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcmi_cr](index.html) module"]
pub struct DCMI_CR_SPEC;
impl crate::RegisterSpec for DCMI_CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcmi_cr::R](R) reader structure"]
impl crate::Readable for DCMI_CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dcmi_cr::W](W) writer structure"]
impl crate::Writable for DCMI_CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DCMI_CR to value 0"]
impl crate::Resettable for DCMI_CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
