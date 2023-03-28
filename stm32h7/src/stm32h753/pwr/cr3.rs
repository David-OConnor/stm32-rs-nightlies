#[doc = "Register `CR3` reader"]
pub struct R(crate::R<CR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR3` writer"]
pub struct W(crate::W<CR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR3_SPEC>;
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
impl From<crate::W<CR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BYPASS` reader - Power management unit bypass"]
pub type BYPASS_R = crate::BitReader<bool>;
#[doc = "Field `BYPASS` writer - Power management unit bypass"]
pub type BYPASS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR3_SPEC, bool, O>;
#[doc = "Field `LDOEN` reader - Low drop-out regulator enable"]
pub type LDOEN_R = crate::BitReader<bool>;
#[doc = "Field `LDOEN` writer - Low drop-out regulator enable"]
pub type LDOEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR3_SPEC, bool, O>;
#[doc = "Field `SCUEN` reader - SD converter Enable"]
pub type SCUEN_R = crate::BitReader<bool>;
#[doc = "Field `SCUEN` writer - SD converter Enable"]
pub type SCUEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR3_SPEC, bool, O>;
#[doc = "Field `VBE` reader - VBAT charging enable"]
pub type VBE_R = crate::BitReader<bool>;
#[doc = "Field `VBE` writer - VBAT charging enable"]
pub type VBE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR3_SPEC, bool, O>;
#[doc = "Field `VBRS` reader - VBAT charging resistor selection"]
pub type VBRS_R = crate::BitReader<bool>;
#[doc = "Field `VBRS` writer - VBAT charging resistor selection"]
pub type VBRS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR3_SPEC, bool, O>;
#[doc = "Field `USB33DEN` reader - VDD33USB voltage level detector enable."]
pub type USB33DEN_R = crate::BitReader<bool>;
#[doc = "Field `USB33DEN` writer - VDD33USB voltage level detector enable."]
pub type USB33DEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR3_SPEC, bool, O>;
#[doc = "Field `USBREGEN` reader - USB regulator enable."]
pub type USBREGEN_R = crate::BitReader<bool>;
#[doc = "Field `USBREGEN` writer - USB regulator enable."]
pub type USBREGEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR3_SPEC, bool, O>;
#[doc = "Field `USB33RDY` reader - USB supply ready."]
pub type USB33RDY_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Power management unit bypass"]
    #[inline(always)]
    pub fn bypass(&self) -> BYPASS_R {
        BYPASS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Low drop-out regulator enable"]
    #[inline(always)]
    pub fn ldoen(&self) -> LDOEN_R {
        LDOEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SD converter Enable"]
    #[inline(always)]
    pub fn scuen(&self) -> SCUEN_R {
        SCUEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - VBAT charging enable"]
    #[inline(always)]
    pub fn vbe(&self) -> VBE_R {
        VBE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - VBAT charging resistor selection"]
    #[inline(always)]
    pub fn vbrs(&self) -> VBRS_R {
        VBRS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 24 - VDD33USB voltage level detector enable."]
    #[inline(always)]
    pub fn usb33den(&self) -> USB33DEN_R {
        USB33DEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - USB regulator enable."]
    #[inline(always)]
    pub fn usbregen(&self) -> USBREGEN_R {
        USBREGEN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - USB supply ready."]
    #[inline(always)]
    pub fn usb33rdy(&self) -> USB33RDY_R {
        USB33RDY_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Power management unit bypass"]
    #[inline(always)]
    #[must_use]
    pub fn bypass(&mut self) -> BYPASS_W<0> {
        BYPASS_W::new(self)
    }
    #[doc = "Bit 1 - Low drop-out regulator enable"]
    #[inline(always)]
    #[must_use]
    pub fn ldoen(&mut self) -> LDOEN_W<1> {
        LDOEN_W::new(self)
    }
    #[doc = "Bit 2 - SD converter Enable"]
    #[inline(always)]
    #[must_use]
    pub fn scuen(&mut self) -> SCUEN_W<2> {
        SCUEN_W::new(self)
    }
    #[doc = "Bit 8 - VBAT charging enable"]
    #[inline(always)]
    #[must_use]
    pub fn vbe(&mut self) -> VBE_W<8> {
        VBE_W::new(self)
    }
    #[doc = "Bit 9 - VBAT charging resistor selection"]
    #[inline(always)]
    #[must_use]
    pub fn vbrs(&mut self) -> VBRS_W<9> {
        VBRS_W::new(self)
    }
    #[doc = "Bit 24 - VDD33USB voltage level detector enable."]
    #[inline(always)]
    #[must_use]
    pub fn usb33den(&mut self) -> USB33DEN_W<24> {
        USB33DEN_W::new(self)
    }
    #[doc = "Bit 25 - USB regulator enable."]
    #[inline(always)]
    #[must_use]
    pub fn usbregen(&mut self) -> USBREGEN_W<25> {
        USBREGEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Reset only by POR only, not reset by wakeup from Standby mode and RESET pad. The lower byte of this register is written once after POR and shall be written before changing VOS level or ck_sys clock frequency. No limitation applies to the upper bytes.Programming data corresponding to an invalid combination of SDLEVEL, SDEXTHP, SDEN, LDOEN and BYPASS bits (see Table9) will be ignored: data will not be written, the written-once mechanism will lock the register and any further write access will be ignored. The default supply configuration will be kept and the ACTVOSRDY bit in PWR control status register 1 (PWR_CSR1) will go on indicating invalid voltage levels. The system shall be power cycled before writing a new value.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr3](index.html) module"]
pub struct CR3_SPEC;
impl crate::RegisterSpec for CR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr3::R](R) reader structure"]
impl crate::Readable for CR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr3::W](W) writer structure"]
impl crate::Writable for CR3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR3 to value 0x06"]
impl crate::Resettable for CR3_SPEC {
    const RESET_VALUE: Self::Ux = 0x06;
}
