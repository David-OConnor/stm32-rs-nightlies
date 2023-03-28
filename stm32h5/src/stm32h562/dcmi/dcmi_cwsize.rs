#[doc = "Register `DCMI_CWSIZE` reader"]
pub struct R(crate::R<DCMI_CWSIZE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCMI_CWSIZE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCMI_CWSIZE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCMI_CWSIZE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCMI_CWSIZE` writer"]
pub struct W(crate::W<DCMI_CWSIZE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCMI_CWSIZE_SPEC>;
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
impl From<crate::W<DCMI_CWSIZE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCMI_CWSIZE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CAPCNT` reader - Capture count This value gives the number of pixel clocks to be captured from the starting point on the same line. It value must corresponds to word-aligned data for different widths of parallel interfaces. 0x0000 => 1 pixel 0x0001 => 2 pixels 0x0002 => 3 pixels ...."]
pub type CAPCNT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CAPCNT` writer - Capture count This value gives the number of pixel clocks to be captured from the starting point on the same line. It value must corresponds to word-aligned data for different widths of parallel interfaces. 0x0000 => 1 pixel 0x0001 => 2 pixels 0x0002 => 3 pixels ...."]
pub type CAPCNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DCMI_CWSIZE_SPEC, u16, u16, 14, O>;
#[doc = "Field `VLINE` reader - Vertical line count This value gives the number of lines to be captured from the starting point. ...."]
pub type VLINE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `VLINE` writer - Vertical line count This value gives the number of lines to be captured from the starting point. ...."]
pub type VLINE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DCMI_CWSIZE_SPEC, u16, u16, 14, O>;
impl R {
    #[doc = "Bits 0:13 - Capture count This value gives the number of pixel clocks to be captured from the starting point on the same line. It value must corresponds to word-aligned data for different widths of parallel interfaces. 0x0000 => 1 pixel 0x0001 => 2 pixels 0x0002 => 3 pixels ...."]
    #[inline(always)]
    pub fn capcnt(&self) -> CAPCNT_R {
        CAPCNT_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 16:29 - Vertical line count This value gives the number of lines to be captured from the starting point. ...."]
    #[inline(always)]
    pub fn vline(&self) -> VLINE_R {
        VLINE_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - Capture count This value gives the number of pixel clocks to be captured from the starting point on the same line. It value must corresponds to word-aligned data for different widths of parallel interfaces. 0x0000 => 1 pixel 0x0001 => 2 pixels 0x0002 => 3 pixels ...."]
    #[inline(always)]
    #[must_use]
    pub fn capcnt(&mut self) -> CAPCNT_W<0> {
        CAPCNT_W::new(self)
    }
    #[doc = "Bits 16:29 - Vertical line count This value gives the number of lines to be captured from the starting point. ...."]
    #[inline(always)]
    #[must_use]
    pub fn vline(&mut self) -> VLINE_W<16> {
        VLINE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DCMI crop window size\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcmi_cwsize](index.html) module"]
pub struct DCMI_CWSIZE_SPEC;
impl crate::RegisterSpec for DCMI_CWSIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcmi_cwsize::R](R) reader structure"]
impl crate::Readable for DCMI_CWSIZE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dcmi_cwsize::W](W) writer structure"]
impl crate::Writable for DCMI_CWSIZE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DCMI_CWSIZE to value 0"]
impl crate::Resettable for DCMI_CWSIZE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
