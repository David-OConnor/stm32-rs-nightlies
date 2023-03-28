#[doc = "Register `SQR1` reader"]
pub struct R(crate::R<SQR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SQR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SQR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SQR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SQR1` writer"]
pub struct W(crate::W<SQR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SQR1_SPEC>;
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
impl From<crate::W<SQR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SQR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SQ25` reader - 25th conversion in regular sequence"]
pub type SQ25_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SQ25` writer - 25th conversion in regular sequence"]
pub type SQ25_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SQR1_SPEC, u8, u8, 5, O>;
#[doc = "Field `SQ26` reader - 26th conversion in regular sequence"]
pub type SQ26_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SQ26` writer - 26th conversion in regular sequence"]
pub type SQ26_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SQR1_SPEC, u8, u8, 5, O>;
#[doc = "Field `SQ27` reader - 27th conversion in regular sequence"]
pub type SQ27_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SQ27` writer - 27th conversion in regular sequence"]
pub type SQ27_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SQR1_SPEC, u8, u8, 5, O>;
#[doc = "Field `SQ28` reader - 28th conversion in regular sequence"]
pub type SQ28_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SQ28` writer - 28th conversion in regular sequence"]
pub type SQ28_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SQR1_SPEC, u8, u8, 5, O>;
#[doc = "Field `L` reader - Regular channel sequence length"]
pub type L_R = crate::FieldReader<u8, u8>;
#[doc = "Field `L` writer - Regular channel sequence length"]
pub type L_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SQR1_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:4 - 25th conversion in regular sequence"]
    #[inline(always)]
    pub fn sq25(&self) -> SQ25_R {
        SQ25_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - 26th conversion in regular sequence"]
    #[inline(always)]
    pub fn sq26(&self) -> SQ26_R {
        SQ26_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - 27th conversion in regular sequence"]
    #[inline(always)]
    pub fn sq27(&self) -> SQ27_R {
        SQ27_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19 - 28th conversion in regular sequence"]
    #[inline(always)]
    pub fn sq28(&self) -> SQ28_R {
        SQ28_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 20:23 - Regular channel sequence length"]
    #[inline(always)]
    pub fn l(&self) -> L_R {
        L_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 25th conversion in regular sequence"]
    #[inline(always)]
    #[must_use]
    pub fn sq25(&mut self) -> SQ25_W<0> {
        SQ25_W::new(self)
    }
    #[doc = "Bits 5:9 - 26th conversion in regular sequence"]
    #[inline(always)]
    #[must_use]
    pub fn sq26(&mut self) -> SQ26_W<5> {
        SQ26_W::new(self)
    }
    #[doc = "Bits 10:14 - 27th conversion in regular sequence"]
    #[inline(always)]
    #[must_use]
    pub fn sq27(&mut self) -> SQ27_W<10> {
        SQ27_W::new(self)
    }
    #[doc = "Bits 15:19 - 28th conversion in regular sequence"]
    #[inline(always)]
    #[must_use]
    pub fn sq28(&mut self) -> SQ28_W<15> {
        SQ28_W::new(self)
    }
    #[doc = "Bits 20:23 - Regular channel sequence length"]
    #[inline(always)]
    #[must_use]
    pub fn l(&mut self) -> L_W<20> {
        L_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "regular sequence register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sqr1](index.html) module"]
pub struct SQR1_SPEC;
impl crate::RegisterSpec for SQR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sqr1::R](R) reader structure"]
impl crate::Readable for SQR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sqr1::W](W) writer structure"]
impl crate::Writable for SQR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SQR1 to value 0"]
impl crate::Resettable for SQR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
