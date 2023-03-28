#[doc = "Register `PMC` reader"]
pub struct R(crate::R<PMC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PMC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PMC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PMC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PMC` writer"]
pub struct W(crate::W<PMC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PMC_SPEC>;
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
impl From<crate::W<PMC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PMC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USB_PU` reader - USB pull-up"]
pub type USB_PU_R = crate::BitReader<bool>;
#[doc = "Field `USB_PU` writer - USB pull-up"]
pub type USB_PU_W<'a, const O: u8> = crate::BitWriter<'a, u32, PMC_SPEC, bool, O>;
#[doc = "Field `LCD_CAPA` reader - decoupling capacitance connection"]
pub type LCD_CAPA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LCD_CAPA` writer - decoupling capacitance connection"]
pub type LCD_CAPA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PMC_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bit 0 - USB pull-up"]
    #[inline(always)]
    pub fn usb_pu(&self) -> USB_PU_R {
        USB_PU_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:5 - decoupling capacitance connection"]
    #[inline(always)]
    pub fn lcd_capa(&self) -> LCD_CAPA_R {
        LCD_CAPA_R::new(((self.bits >> 1) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - USB pull-up"]
    #[inline(always)]
    #[must_use]
    pub fn usb_pu(&mut self) -> USB_PU_W<0> {
        USB_PU_W::new(self)
    }
    #[doc = "Bits 1:5 - decoupling capacitance connection"]
    #[inline(always)]
    #[must_use]
    pub fn lcd_capa(&mut self) -> LCD_CAPA_W<1> {
        LCD_CAPA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "peripheral mode configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmc](index.html) module"]
pub struct PMC_SPEC;
impl crate::RegisterSpec for PMC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pmc::R](R) reader structure"]
impl crate::Readable for PMC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pmc::W](W) writer structure"]
impl crate::Writable for PMC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PMC to value 0"]
impl crate::Resettable for PMC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
