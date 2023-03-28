#[doc = "Register `PARAM` reader"]
pub struct R(crate::R<PARAM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PARAM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PARAM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PARAM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PARAM` writer"]
pub struct W(crate::W<PARAM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PARAM_SPEC>;
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
impl From<crate::W<PARAM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PARAM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P` reader - P"]
pub type P_R = crate::FieldReader<u8, u8>;
#[doc = "Field `P` writer - P"]
pub type P_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PARAM_SPEC, u8, u8, 8, O>;
#[doc = "Field `Q` reader - Q"]
pub type Q_R = crate::FieldReader<u8, u8>;
#[doc = "Field `Q` writer - Q"]
pub type Q_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PARAM_SPEC, u8, u8, 8, O>;
#[doc = "Field `R` reader - R"]
pub type R_R = crate::FieldReader<u8, u8>;
#[doc = "Field `R` writer - R"]
pub type R_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PARAM_SPEC, u8, u8, 8, O>;
#[doc = "Field `FUNC` reader - FUNC"]
pub type FUNC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FUNC` writer - FUNC"]
pub type FUNC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PARAM_SPEC, u8, u8, 7, O>;
#[doc = "Field `START` reader - START"]
pub type START_R = crate::BitReader<bool>;
#[doc = "Field `START` writer - START"]
pub type START_W<'a, const O: u8> = crate::BitWriter<'a, u32, PARAM_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:7 - P"]
    #[inline(always)]
    pub fn p(&self) -> P_R {
        P_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Q"]
    #[inline(always)]
    pub fn q(&self) -> Q_R {
        Q_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - R"]
    #[inline(always)]
    pub fn r(&self) -> R_R {
        R_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:30 - FUNC"]
    #[inline(always)]
    pub fn func(&self) -> FUNC_R {
        FUNC_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
    #[doc = "Bit 31 - START"]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - P"]
    #[inline(always)]
    #[must_use]
    pub fn p(&mut self) -> P_W<0> {
        P_W::new(self)
    }
    #[doc = "Bits 8:15 - Q"]
    #[inline(always)]
    #[must_use]
    pub fn q(&mut self) -> Q_W<8> {
        Q_W::new(self)
    }
    #[doc = "Bits 16:23 - R"]
    #[inline(always)]
    #[must_use]
    pub fn r(&mut self) -> R_W<16> {
        R_W::new(self)
    }
    #[doc = "Bits 24:30 - FUNC"]
    #[inline(always)]
    #[must_use]
    pub fn func(&mut self) -> FUNC_W<24> {
        FUNC_W::new(self)
    }
    #[doc = "Bit 31 - START"]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<31> {
        START_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FMAC Parameter register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [param](index.html) module"]
pub struct PARAM_SPEC;
impl crate::RegisterSpec for PARAM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [param::R](R) reader structure"]
impl crate::Readable for PARAM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [param::W](W) writer structure"]
impl crate::Writable for PARAM_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PARAM to value 0"]
impl crate::Resettable for PARAM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
