#[doc = "Register `MACLMIR` reader"]
pub struct R(crate::R<MACLMIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACLMIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACLMIR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACLMIR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MACLMIR` writer"]
pub struct W(crate::W<MACLMIR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MACLMIR_SPEC>;
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
impl From<crate::W<MACLMIR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MACLMIR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LSI` reader - Log Sync Interval"]
pub type LSI_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LSI` writer - Log Sync Interval"]
pub type LSI_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MACLMIR_SPEC, u8, u8, 8, O>;
#[doc = "Field `DRSYNCR` reader - Delay_Req to SYNC Ratio"]
pub type DRSYNCR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DRSYNCR` writer - Delay_Req to SYNC Ratio"]
pub type DRSYNCR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MACLMIR_SPEC, u8, u8, 3, O>;
#[doc = "Field `LMPDRI` reader - Log Min Pdelay_Req Interval"]
pub type LMPDRI_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LMPDRI` writer - Log Min Pdelay_Req Interval"]
pub type LMPDRI_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MACLMIR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Log Sync Interval"]
    #[inline(always)]
    pub fn lsi(&self) -> LSI_R {
        LSI_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:10 - Delay_Req to SYNC Ratio"]
    #[inline(always)]
    pub fn drsyncr(&self) -> DRSYNCR_R {
        DRSYNCR_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 24:31 - Log Min Pdelay_Req Interval"]
    #[inline(always)]
    pub fn lmpdri(&self) -> LMPDRI_R {
        LMPDRI_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Log Sync Interval"]
    #[inline(always)]
    #[must_use]
    pub fn lsi(&mut self) -> LSI_W<0> {
        LSI_W::new(self)
    }
    #[doc = "Bits 8:10 - Delay_Req to SYNC Ratio"]
    #[inline(always)]
    #[must_use]
    pub fn drsyncr(&mut self) -> DRSYNCR_W<8> {
        DRSYNCR_W::new(self)
    }
    #[doc = "Bits 24:31 - Log Min Pdelay_Req Interval"]
    #[inline(always)]
    #[must_use]
    pub fn lmpdri(&mut self) -> LMPDRI_W<24> {
        LMPDRI_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Log message interval register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [maclmir](index.html) module"]
pub struct MACLMIR_SPEC;
impl crate::RegisterSpec for MACLMIR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [maclmir::R](R) reader structure"]
impl crate::Readable for MACLMIR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [maclmir::W](W) writer structure"]
impl crate::Writable for MACLMIR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MACLMIR to value 0"]
impl crate::Resettable for MACLMIR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
