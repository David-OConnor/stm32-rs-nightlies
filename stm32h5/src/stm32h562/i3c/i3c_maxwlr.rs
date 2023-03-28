#[doc = "Register `I3C_MAXWLR` reader"]
pub struct R(crate::R<I3C_MAXWLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I3C_MAXWLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I3C_MAXWLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I3C_MAXWLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I3C_MAXWLR` writer"]
pub struct W(crate::W<I3C_MAXWLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I3C_MAXWLR_SPEC>;
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
impl From<crate::W<I3C_MAXWLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I3C_MAXWLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MWL` reader - maximum data write length (when I3C is acting as target) This field is initially written by software when I3C_CFGR.EN=0 and updated by hardware on the reception of SETMWL command. Software is notified of a MWL update by the I3C_EVR.MWLUPF and the corresponding interrupt if enabled. This field is used by hardware to return the value on the I3C bus when the target receives a GETMWL CCC."]
pub type MWL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `MWL` writer - maximum data write length (when I3C is acting as target) This field is initially written by software when I3C_CFGR.EN=0 and updated by hardware on the reception of SETMWL command. Software is notified of a MWL update by the I3C_EVR.MWLUPF and the corresponding interrupt if enabled. This field is used by hardware to return the value on the I3C bus when the target receives a GETMWL CCC."]
pub type MWL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, I3C_MAXWLR_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - maximum data write length (when I3C is acting as target) This field is initially written by software when I3C_CFGR.EN=0 and updated by hardware on the reception of SETMWL command. Software is notified of a MWL update by the I3C_EVR.MWLUPF and the corresponding interrupt if enabled. This field is used by hardware to return the value on the I3C bus when the target receives a GETMWL CCC."]
    #[inline(always)]
    pub fn mwl(&self) -> MWL_R {
        MWL_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - maximum data write length (when I3C is acting as target) This field is initially written by software when I3C_CFGR.EN=0 and updated by hardware on the reception of SETMWL command. Software is notified of a MWL update by the I3C_EVR.MWLUPF and the corresponding interrupt if enabled. This field is used by hardware to return the value on the I3C bus when the target receives a GETMWL CCC."]
    #[inline(always)]
    #[must_use]
    pub fn mwl(&mut self) -> MWL_W<0> {
        MWL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I3C maximum write length register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i3c_maxwlr](index.html) module"]
pub struct I3C_MAXWLR_SPEC;
impl crate::RegisterSpec for I3C_MAXWLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i3c_maxwlr::R](R) reader structure"]
impl crate::Readable for I3C_MAXWLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i3c_maxwlr::W](W) writer structure"]
impl crate::Writable for I3C_MAXWLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets I3C_MAXWLR to value 0"]
impl crate::Resettable for I3C_MAXWLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
