#[doc = "Register `CONFCHR1` reader"]
pub struct R(crate::R<CONFCHR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONFCHR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONFCHR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONFCHR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONFCHR1` writer"]
pub struct W(crate::W<CONFCHR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONFCHR1_SPEC>;
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
impl From<crate::W<CONFCHR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONFCHR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CONFCH0` reader - CONFCH0"]
pub type CONFCH0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CONFCH0` writer - CONFCH0"]
pub type CONFCH0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CONFCHR1_SPEC, u8, u8, 2, O>;
#[doc = "Field `CONFCH1` reader - CONFCH1"]
pub type CONFCH1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CONFCH1` writer - CONFCH1"]
pub type CONFCH1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CONFCHR1_SPEC, u8, u8, 2, O>;
#[doc = "Field `CONFCH2` reader - CONFCH2"]
pub type CONFCH2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CONFCH2` writer - CONFCH2"]
pub type CONFCH2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CONFCHR1_SPEC, u8, u8, 2, O>;
#[doc = "Field `CONFCH3` reader - CONFCH3"]
pub type CONFCH3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CONFCH3` writer - CONFCH3"]
pub type CONFCH3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CONFCHR1_SPEC, u8, u8, 2, O>;
#[doc = "Field `CONFCH4` reader - CONFCH4"]
pub type CONFCH4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CONFCH4` writer - CONFCH4"]
pub type CONFCH4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CONFCHR1_SPEC, u8, u8, 2, O>;
#[doc = "Field `CONFCH5` reader - CONFCH5"]
pub type CONFCH5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CONFCH5` writer - CONFCH5"]
pub type CONFCH5_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CONFCHR1_SPEC, u8, u8, 2, O>;
#[doc = "Field `CONFCH6` reader - CONFCH6"]
pub type CONFCH6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CONFCH6` writer - CONFCH6"]
pub type CONFCH6_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CONFCHR1_SPEC, u8, u8, 2, O>;
#[doc = "Field `CONFCH7` reader - CONFCH7"]
pub type CONFCH7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CONFCH7` writer - CONFCH7"]
pub type CONFCH7_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CONFCHR1_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1 - CONFCH0"]
    #[inline(always)]
    pub fn confch0(&self) -> CONFCH0_R {
        CONFCH0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - CONFCH1"]
    #[inline(always)]
    pub fn confch1(&self) -> CONFCH1_R {
        CONFCH1_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:9 - CONFCH2"]
    #[inline(always)]
    pub fn confch2(&self) -> CONFCH2_R {
        CONFCH2_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:13 - CONFCH3"]
    #[inline(always)]
    pub fn confch3(&self) -> CONFCH3_R {
        CONFCH3_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:17 - CONFCH4"]
    #[inline(always)]
    pub fn confch4(&self) -> CONFCH4_R {
        CONFCH4_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 20:21 - CONFCH5"]
    #[inline(always)]
    pub fn confch5(&self) -> CONFCH5_R {
        CONFCH5_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 24:25 - CONFCH6"]
    #[inline(always)]
    pub fn confch6(&self) -> CONFCH6_R {
        CONFCH6_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 28:29 - CONFCH7"]
    #[inline(always)]
    pub fn confch7(&self) -> CONFCH7_R {
        CONFCH7_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - CONFCH0"]
    #[inline(always)]
    #[must_use]
    pub fn confch0(&mut self) -> CONFCH0_W<0> {
        CONFCH0_W::new(self)
    }
    #[doc = "Bits 4:5 - CONFCH1"]
    #[inline(always)]
    #[must_use]
    pub fn confch1(&mut self) -> CONFCH1_W<4> {
        CONFCH1_W::new(self)
    }
    #[doc = "Bits 8:9 - CONFCH2"]
    #[inline(always)]
    #[must_use]
    pub fn confch2(&mut self) -> CONFCH2_W<8> {
        CONFCH2_W::new(self)
    }
    #[doc = "Bits 12:13 - CONFCH3"]
    #[inline(always)]
    #[must_use]
    pub fn confch3(&mut self) -> CONFCH3_W<12> {
        CONFCH3_W::new(self)
    }
    #[doc = "Bits 16:17 - CONFCH4"]
    #[inline(always)]
    #[must_use]
    pub fn confch4(&mut self) -> CONFCH4_W<16> {
        CONFCH4_W::new(self)
    }
    #[doc = "Bits 20:21 - CONFCH5"]
    #[inline(always)]
    #[must_use]
    pub fn confch5(&mut self) -> CONFCH5_W<20> {
        CONFCH5_W::new(self)
    }
    #[doc = "Bits 24:25 - CONFCH6"]
    #[inline(always)]
    #[must_use]
    pub fn confch6(&mut self) -> CONFCH6_W<24> {
        CONFCH6_W::new(self)
    }
    #[doc = "Bits 28:29 - CONFCH7"]
    #[inline(always)]
    #[must_use]
    pub fn confch7(&mut self) -> CONFCH7_W<28> {
        CONFCH7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "channel configuration register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [confchr1](index.html) module"]
pub struct CONFCHR1_SPEC;
impl crate::RegisterSpec for CONFCHR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [confchr1::R](R) reader structure"]
impl crate::Readable for CONFCHR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [confchr1::W](W) writer structure"]
impl crate::Writable for CONFCHR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CONFCHR1 to value 0"]
impl crate::Resettable for CONFCHR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
