#[doc = "Register `GICD_SPENDSGIR1` reader"]
pub struct R(crate::R<GICD_SPENDSGIR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_SPENDSGIR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_SPENDSGIR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_SPENDSGIR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GICD_SPENDSGIR1` writer"]
pub struct W(crate::W<GICD_SPENDSGIR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GICD_SPENDSGIR1_SPEC>;
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
impl From<crate::W<GICD_SPENDSGIR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GICD_SPENDSGIR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SGI_SET_PENDING0` reader - SGI_SET_PENDING0"]
pub type SGI_SET_PENDING0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SGI_SET_PENDING0` writer - SGI_SET_PENDING0"]
pub type SGI_SET_PENDING0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_SPENDSGIR1_SPEC, u8, u8, 2, O>;
#[doc = "Field `SGI_SET_PENDING1` reader - SGI_SET_PENDING1"]
pub type SGI_SET_PENDING1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SGI_SET_PENDING1` writer - SGI_SET_PENDING1"]
pub type SGI_SET_PENDING1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_SPENDSGIR1_SPEC, u8, u8, 2, O>;
#[doc = "Field `SGI_SET_PENDING2` reader - SGI_SET_PENDING2"]
pub type SGI_SET_PENDING2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SGI_SET_PENDING2` writer - SGI_SET_PENDING2"]
pub type SGI_SET_PENDING2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_SPENDSGIR1_SPEC, u8, u8, 2, O>;
#[doc = "Field `SGI_SET_PENDING3` reader - SGI_SET_PENDING3"]
pub type SGI_SET_PENDING3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SGI_SET_PENDING3` writer - SGI_SET_PENDING3"]
pub type SGI_SET_PENDING3_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_SPENDSGIR1_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1 - SGI_SET_PENDING0"]
    #[inline(always)]
    pub fn sgi_set_pending0(&self) -> SGI_SET_PENDING0_R {
        SGI_SET_PENDING0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:9 - SGI_SET_PENDING1"]
    #[inline(always)]
    pub fn sgi_set_pending1(&self) -> SGI_SET_PENDING1_R {
        SGI_SET_PENDING1_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:17 - SGI_SET_PENDING2"]
    #[inline(always)]
    pub fn sgi_set_pending2(&self) -> SGI_SET_PENDING2_R {
        SGI_SET_PENDING2_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 24:25 - SGI_SET_PENDING3"]
    #[inline(always)]
    pub fn sgi_set_pending3(&self) -> SGI_SET_PENDING3_R {
        SGI_SET_PENDING3_R::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - SGI_SET_PENDING0"]
    #[inline(always)]
    #[must_use]
    pub fn sgi_set_pending0(&mut self) -> SGI_SET_PENDING0_W<0> {
        SGI_SET_PENDING0_W::new(self)
    }
    #[doc = "Bits 8:9 - SGI_SET_PENDING1"]
    #[inline(always)]
    #[must_use]
    pub fn sgi_set_pending1(&mut self) -> SGI_SET_PENDING1_W<8> {
        SGI_SET_PENDING1_W::new(self)
    }
    #[doc = "Bits 16:17 - SGI_SET_PENDING2"]
    #[inline(always)]
    #[must_use]
    pub fn sgi_set_pending2(&mut self) -> SGI_SET_PENDING2_W<16> {
        SGI_SET_PENDING2_W::new(self)
    }
    #[doc = "Bits 24:25 - SGI_SET_PENDING3"]
    #[inline(always)]
    #[must_use]
    pub fn sgi_set_pending3(&mut self) -> SGI_SET_PENDING3_W<24> {
        SGI_SET_PENDING3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "For SGI x*4 to SGI x*4+3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_spendsgir1](index.html) module"]
pub struct GICD_SPENDSGIR1_SPEC;
impl crate::RegisterSpec for GICD_SPENDSGIR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicd_spendsgir1::R](R) reader structure"]
impl crate::Readable for GICD_SPENDSGIR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gicd_spendsgir1::W](W) writer structure"]
impl crate::Writable for GICD_SPENDSGIR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICD_SPENDSGIR1 to value 0"]
impl crate::Resettable for GICD_SPENDSGIR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
