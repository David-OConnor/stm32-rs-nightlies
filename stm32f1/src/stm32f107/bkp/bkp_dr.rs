#[doc = "Register `BKP_DR%s` reader"]
pub struct R(crate::R<BKP_DR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BKP_DR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BKP_DR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BKP_DR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BKP_DR%s` writer"]
pub struct W(crate::W<BKP_DR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BKP_DR_SPEC>;
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
impl From<crate::W<BKP_DR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BKP_DR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `D` reader - Backup data"]
pub type D_R = crate::FieldReader<u16, u16>;
#[doc = "Field `D` writer - Backup data"]
pub type D_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, BKP_DR_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    pub fn d(&self) -> D_R {
        D_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    #[must_use]
    pub fn d(&mut self) -> D_W<0> {
        D_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Backup data register (BKP_DR)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bkp_dr](index.html) module"]
pub struct BKP_DR_SPEC;
impl crate::RegisterSpec for BKP_DR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bkp_dr::R](R) reader structure"]
impl crate::Readable for BKP_DR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bkp_dr::W](W) writer structure"]
impl crate::Writable for BKP_DR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BKP_DR%s to value 0"]
impl crate::Resettable for BKP_DR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
