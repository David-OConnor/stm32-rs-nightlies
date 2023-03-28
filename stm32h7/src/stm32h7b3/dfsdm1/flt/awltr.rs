#[doc = "Register `AWLTR` reader"]
pub struct R(crate::R<AWLTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AWLTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AWLTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AWLTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AWLTR` writer"]
pub struct W(crate::W<AWLTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AWLTR_SPEC>;
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
impl From<crate::W<AWLTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AWLTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BKAWL` reader - Break signal assignment to analog watchdog low threshold event BKAWL\\[i\\]
= 0: Break i signal is not assigned to an analog watchdog low threshold event BKAWL\\[i\\]
= 1: Break i signal is assigned to an analog watchdog low threshold event"]
pub type BKAWL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BKAWL` writer - Break signal assignment to analog watchdog low threshold event BKAWL\\[i\\]
= 0: Break i signal is not assigned to an analog watchdog low threshold event BKAWL\\[i\\]
= 1: Break i signal is assigned to an analog watchdog low threshold event"]
pub type BKAWL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AWLTR_SPEC, u8, u8, 4, O>;
#[doc = "Field `AWLT` reader - Analog watchdog low threshold These bits are written by software to define the low threshold for the analog watchdog. Note: In case channel transceivers monitor (AWFSEL=1), only the higher 16 bits (AWLT\\[23:8\\]) define the 16-bit threshold as compared with the analog watchdog filter output (because data coming from the analog watchdog filter are up to a 16-bit resolution). Bits AWLT\\[7:0\\]
are not taken into comparison in this case."]
pub type AWLT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `AWLT` writer - Analog watchdog low threshold These bits are written by software to define the low threshold for the analog watchdog. Note: In case channel transceivers monitor (AWFSEL=1), only the higher 16 bits (AWLT\\[23:8\\]) define the 16-bit threshold as compared with the analog watchdog filter output (because data coming from the analog watchdog filter are up to a 16-bit resolution). Bits AWLT\\[7:0\\]
are not taken into comparison in this case."]
pub type AWLT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AWLTR_SPEC, u32, u32, 24, O>;
impl R {
    #[doc = "Bits 0:3 - Break signal assignment to analog watchdog low threshold event BKAWL\\[i\\]
= 0: Break i signal is not assigned to an analog watchdog low threshold event BKAWL\\[i\\]
= 1: Break i signal is assigned to an analog watchdog low threshold event"]
    #[inline(always)]
    pub fn bkawl(&self) -> BKAWL_R {
        BKAWL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:31 - Analog watchdog low threshold These bits are written by software to define the low threshold for the analog watchdog. Note: In case channel transceivers monitor (AWFSEL=1), only the higher 16 bits (AWLT\\[23:8\\]) define the 16-bit threshold as compared with the analog watchdog filter output (because data coming from the analog watchdog filter are up to a 16-bit resolution). Bits AWLT\\[7:0\\]
are not taken into comparison in this case."]
    #[inline(always)]
    pub fn awlt(&self) -> AWLT_R {
        AWLT_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:3 - Break signal assignment to analog watchdog low threshold event BKAWL\\[i\\]
= 0: Break i signal is not assigned to an analog watchdog low threshold event BKAWL\\[i\\]
= 1: Break i signal is assigned to an analog watchdog low threshold event"]
    #[inline(always)]
    #[must_use]
    pub fn bkawl(&mut self) -> BKAWL_W<0> {
        BKAWL_W::new(self)
    }
    #[doc = "Bits 8:31 - Analog watchdog low threshold These bits are written by software to define the low threshold for the analog watchdog. Note: In case channel transceivers monitor (AWFSEL=1), only the higher 16 bits (AWLT\\[23:8\\]) define the 16-bit threshold as compared with the analog watchdog filter output (because data coming from the analog watchdog filter are up to a 16-bit resolution). Bits AWLT\\[7:0\\]
are not taken into comparison in this case."]
    #[inline(always)]
    #[must_use]
    pub fn awlt(&mut self) -> AWLT_W<8> {
        AWLT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [awltr](index.html) module"]
pub struct AWLTR_SPEC;
impl crate::RegisterSpec for AWLTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [awltr::R](R) reader structure"]
impl crate::Readable for AWLTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [awltr::W](W) writer structure"]
impl crate::Writable for AWLTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AWLTR to value 0"]
impl crate::Resettable for AWLTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
