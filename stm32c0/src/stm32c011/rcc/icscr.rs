#[doc = "Register `ICSCR` reader"]
pub struct R(crate::R<ICSCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ICSCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ICSCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ICSCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ICSCR` writer"]
pub struct W(crate::W<ICSCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICSCR_SPEC>;
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
impl From<crate::W<ICSCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICSCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HSICAL` reader - HSI48 clock calibration This bitfield directly acts on the HSI48 clock frequency. Its value is a sum of an internal factory-programmed number and the value of the HSITRIM\\[6:0\\]
bitfield. In the factory, the internal number is set to calibrate the HSI48 clock frequency to 48 MHz (with HSITRIM\\[6:0\\]
left at its reset value). Refer to the device datasheet for HSI48 calibration accuracy and for the frequency trimming granularity. Note: The trimming effect presents discontinuities at HSICAL\\[7:0\\]
multiples of 64."]
pub type HSICAL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HSITRIM` reader - HSI48 clock trimming The value of this bitfield contributes to the HSICAL\\[7:0\\]
bitfield value. It allows HSI48 clock frequency user trimming. The HSI48 frequency accuracy as stated in the device datasheet applies when this bitfield is left at its reset value."]
pub type HSITRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HSITRIM` writer - HSI48 clock trimming The value of this bitfield contributes to the HSICAL\\[7:0\\]
bitfield value. It allows HSI48 clock frequency user trimming. The HSI48 frequency accuracy as stated in the device datasheet applies when this bitfield is left at its reset value."]
pub type HSITRIM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ICSCR_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 0:7 - HSI48 clock calibration This bitfield directly acts on the HSI48 clock frequency. Its value is a sum of an internal factory-programmed number and the value of the HSITRIM\\[6:0\\]
bitfield. In the factory, the internal number is set to calibrate the HSI48 clock frequency to 48 MHz (with HSITRIM\\[6:0\\]
left at its reset value). Refer to the device datasheet for HSI48 calibration accuracy and for the frequency trimming granularity. Note: The trimming effect presents discontinuities at HSICAL\\[7:0\\]
multiples of 64."]
    #[inline(always)]
    pub fn hsical(&self) -> HSICAL_R {
        HSICAL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:14 - HSI48 clock trimming The value of this bitfield contributes to the HSICAL\\[7:0\\]
bitfield value. It allows HSI48 clock frequency user trimming. The HSI48 frequency accuracy as stated in the device datasheet applies when this bitfield is left at its reset value."]
    #[inline(always)]
    pub fn hsitrim(&self) -> HSITRIM_R {
        HSITRIM_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 8:14 - HSI48 clock trimming The value of this bitfield contributes to the HSICAL\\[7:0\\]
bitfield value. It allows HSI48 clock frequency user trimming. The HSI48 frequency accuracy as stated in the device datasheet applies when this bitfield is left at its reset value."]
    #[inline(always)]
    #[must_use]
    pub fn hsitrim(&mut self) -> HSITRIM_W<8> {
        HSITRIM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RCC internal clock source calibration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icscr](index.html) module"]
pub struct ICSCR_SPEC;
impl crate::RegisterSpec for ICSCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [icscr::R](R) reader structure"]
impl crate::Readable for ICSCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [icscr::W](W) writer structure"]
impl crate::Writable for ICSCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ICSCR to value 0x4000"]
impl crate::Resettable for ICSCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x4000;
}
