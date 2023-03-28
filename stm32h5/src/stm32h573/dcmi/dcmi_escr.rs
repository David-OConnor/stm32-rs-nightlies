#[doc = "Register `DCMI_ESCR` reader"]
pub struct R(crate::R<DCMI_ESCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCMI_ESCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCMI_ESCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCMI_ESCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCMI_ESCR` writer"]
pub struct W(crate::W<DCMI_ESCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCMI_ESCR_SPEC>;
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
impl From<crate::W<DCMI_ESCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCMI_ESCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FSC` reader - Frame start delimiter code This byte specifies the code of the frame start delimiter. The code consists of 4 bytes in the form of 0xFF, 0x00, 0x00, FSC. If FSC is programmed to 0xFF, no frame start delimiter is detected. But, the first occurrence of LSC after an FEC code is interpreted as a start of frame delimiter."]
pub type FSC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FSC` writer - Frame start delimiter code This byte specifies the code of the frame start delimiter. The code consists of 4 bytes in the form of 0xFF, 0x00, 0x00, FSC. If FSC is programmed to 0xFF, no frame start delimiter is detected. But, the first occurrence of LSC after an FEC code is interpreted as a start of frame delimiter."]
pub type FSC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DCMI_ESCR_SPEC, u8, u8, 8, O>;
#[doc = "Field `LSC` reader - Line start delimiter code This byte specifies the code of the line start delimiter. The code consists of 4 bytes in the form of 0xFF, 0x00, 0x00, LSC."]
pub type LSC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LSC` writer - Line start delimiter code This byte specifies the code of the line start delimiter. The code consists of 4 bytes in the form of 0xFF, 0x00, 0x00, LSC."]
pub type LSC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DCMI_ESCR_SPEC, u8, u8, 8, O>;
#[doc = "Field `LEC` reader - Line end delimiter code This byte specifies the code of the line end delimiter. The code consists of 4 bytes in the form of 0xFF, 0x00, 0x00, LEC."]
pub type LEC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LEC` writer - Line end delimiter code This byte specifies the code of the line end delimiter. The code consists of 4 bytes in the form of 0xFF, 0x00, 0x00, LEC."]
pub type LEC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DCMI_ESCR_SPEC, u8, u8, 8, O>;
#[doc = "Field `FEC` reader - Frame end delimiter code This byte specifies the code of the frame end delimiter. The code consists of 4 bytes in the form of 0xFF, 0x00, 0x00, FEC. If FEC is programmed to 0xFF, all the unused codes (0xFF0000XY) are interpreted as frame end delimiters."]
pub type FEC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FEC` writer - Frame end delimiter code This byte specifies the code of the frame end delimiter. The code consists of 4 bytes in the form of 0xFF, 0x00, 0x00, FEC. If FEC is programmed to 0xFF, all the unused codes (0xFF0000XY) are interpreted as frame end delimiters."]
pub type FEC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DCMI_ESCR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Frame start delimiter code This byte specifies the code of the frame start delimiter. The code consists of 4 bytes in the form of 0xFF, 0x00, 0x00, FSC. If FSC is programmed to 0xFF, no frame start delimiter is detected. But, the first occurrence of LSC after an FEC code is interpreted as a start of frame delimiter."]
    #[inline(always)]
    pub fn fsc(&self) -> FSC_R {
        FSC_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Line start delimiter code This byte specifies the code of the line start delimiter. The code consists of 4 bytes in the form of 0xFF, 0x00, 0x00, LSC."]
    #[inline(always)]
    pub fn lsc(&self) -> LSC_R {
        LSC_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Line end delimiter code This byte specifies the code of the line end delimiter. The code consists of 4 bytes in the form of 0xFF, 0x00, 0x00, LEC."]
    #[inline(always)]
    pub fn lec(&self) -> LEC_R {
        LEC_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Frame end delimiter code This byte specifies the code of the frame end delimiter. The code consists of 4 bytes in the form of 0xFF, 0x00, 0x00, FEC. If FEC is programmed to 0xFF, all the unused codes (0xFF0000XY) are interpreted as frame end delimiters."]
    #[inline(always)]
    pub fn fec(&self) -> FEC_R {
        FEC_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Frame start delimiter code This byte specifies the code of the frame start delimiter. The code consists of 4 bytes in the form of 0xFF, 0x00, 0x00, FSC. If FSC is programmed to 0xFF, no frame start delimiter is detected. But, the first occurrence of LSC after an FEC code is interpreted as a start of frame delimiter."]
    #[inline(always)]
    #[must_use]
    pub fn fsc(&mut self) -> FSC_W<0> {
        FSC_W::new(self)
    }
    #[doc = "Bits 8:15 - Line start delimiter code This byte specifies the code of the line start delimiter. The code consists of 4 bytes in the form of 0xFF, 0x00, 0x00, LSC."]
    #[inline(always)]
    #[must_use]
    pub fn lsc(&mut self) -> LSC_W<8> {
        LSC_W::new(self)
    }
    #[doc = "Bits 16:23 - Line end delimiter code This byte specifies the code of the line end delimiter. The code consists of 4 bytes in the form of 0xFF, 0x00, 0x00, LEC."]
    #[inline(always)]
    #[must_use]
    pub fn lec(&mut self) -> LEC_W<16> {
        LEC_W::new(self)
    }
    #[doc = "Bits 24:31 - Frame end delimiter code This byte specifies the code of the frame end delimiter. The code consists of 4 bytes in the form of 0xFF, 0x00, 0x00, FEC. If FEC is programmed to 0xFF, all the unused codes (0xFF0000XY) are interpreted as frame end delimiters."]
    #[inline(always)]
    #[must_use]
    pub fn fec(&mut self) -> FEC_W<24> {
        FEC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DCMI embedded synchronization code register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcmi_escr](index.html) module"]
pub struct DCMI_ESCR_SPEC;
impl crate::RegisterSpec for DCMI_ESCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcmi_escr::R](R) reader structure"]
impl crate::Readable for DCMI_ESCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dcmi_escr::W](W) writer structure"]
impl crate::Writable for DCMI_ESCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DCMI_ESCR to value 0"]
impl crate::Resettable for DCMI_ESCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
