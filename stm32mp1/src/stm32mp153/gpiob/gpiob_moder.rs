#[doc = "Register `GPIOB_MODER` reader"]
pub struct R(crate::R<GPIOB_MODER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIOB_MODER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIOB_MODER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIOB_MODER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPIOB_MODER` writer"]
pub struct W(crate::W<GPIOB_MODER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIOB_MODER_SPEC>;
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
impl From<crate::W<GPIOB_MODER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIOB_MODER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MODER0` reader - MODER0"]
pub type MODER0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MODER0` writer - MODER0"]
pub type MODER0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GPIOB_MODER_SPEC, u8, u8, 2, O>;
#[doc = "Field `MODER1` reader - MODER1"]
pub type MODER1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MODER1` writer - MODER1"]
pub type MODER1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GPIOB_MODER_SPEC, u8, u8, 2, O>;
#[doc = "Field `MODER2` reader - MODER2"]
pub type MODER2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MODER2` writer - MODER2"]
pub type MODER2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GPIOB_MODER_SPEC, u8, u8, 2, O>;
#[doc = "Field `MODER3` reader - MODER3"]
pub type MODER3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MODER3` writer - MODER3"]
pub type MODER3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GPIOB_MODER_SPEC, u8, u8, 2, O>;
#[doc = "Field `MODER4` reader - MODER4"]
pub type MODER4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MODER4` writer - MODER4"]
pub type MODER4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GPIOB_MODER_SPEC, u8, u8, 2, O>;
#[doc = "Field `MODER5` reader - MODER5"]
pub type MODER5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MODER5` writer - MODER5"]
pub type MODER5_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GPIOB_MODER_SPEC, u8, u8, 2, O>;
#[doc = "Field `MODER6` reader - MODER6"]
pub type MODER6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MODER6` writer - MODER6"]
pub type MODER6_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GPIOB_MODER_SPEC, u8, u8, 2, O>;
#[doc = "Field `MODER7` reader - MODER7"]
pub type MODER7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MODER7` writer - MODER7"]
pub type MODER7_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GPIOB_MODER_SPEC, u8, u8, 2, O>;
#[doc = "Field `MODER8` reader - MODER8"]
pub type MODER8_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MODER8` writer - MODER8"]
pub type MODER8_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GPIOB_MODER_SPEC, u8, u8, 2, O>;
#[doc = "Field `MODER9` reader - MODER9"]
pub type MODER9_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MODER9` writer - MODER9"]
pub type MODER9_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GPIOB_MODER_SPEC, u8, u8, 2, O>;
#[doc = "Field `MODER10` reader - MODER10"]
pub type MODER10_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MODER10` writer - MODER10"]
pub type MODER10_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GPIOB_MODER_SPEC, u8, u8, 2, O>;
#[doc = "Field `MODER11` reader - MODER11"]
pub type MODER11_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MODER11` writer - MODER11"]
pub type MODER11_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GPIOB_MODER_SPEC, u8, u8, 2, O>;
#[doc = "Field `MODER12` reader - MODER12"]
pub type MODER12_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MODER12` writer - MODER12"]
pub type MODER12_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GPIOB_MODER_SPEC, u8, u8, 2, O>;
#[doc = "Field `MODER13` reader - MODER13"]
pub type MODER13_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MODER13` writer - MODER13"]
pub type MODER13_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GPIOB_MODER_SPEC, u8, u8, 2, O>;
#[doc = "Field `MODER14` reader - MODER14"]
pub type MODER14_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MODER14` writer - MODER14"]
pub type MODER14_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GPIOB_MODER_SPEC, u8, u8, 2, O>;
#[doc = "Field `MODER15` reader - MODER15"]
pub type MODER15_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MODER15` writer - MODER15"]
pub type MODER15_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GPIOB_MODER_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1 - MODER0"]
    #[inline(always)]
    pub fn moder0(&self) -> MODER0_R {
        MODER0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - MODER1"]
    #[inline(always)]
    pub fn moder1(&self) -> MODER1_R {
        MODER1_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - MODER2"]
    #[inline(always)]
    pub fn moder2(&self) -> MODER2_R {
        MODER2_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - MODER3"]
    #[inline(always)]
    pub fn moder3(&self) -> MODER3_R {
        MODER3_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - MODER4"]
    #[inline(always)]
    pub fn moder4(&self) -> MODER4_R {
        MODER4_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - MODER5"]
    #[inline(always)]
    pub fn moder5(&self) -> MODER5_R {
        MODER5_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - MODER6"]
    #[inline(always)]
    pub fn moder6(&self) -> MODER6_R {
        MODER6_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - MODER7"]
    #[inline(always)]
    pub fn moder7(&self) -> MODER7_R {
        MODER7_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - MODER8"]
    #[inline(always)]
    pub fn moder8(&self) -> MODER8_R {
        MODER8_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - MODER9"]
    #[inline(always)]
    pub fn moder9(&self) -> MODER9_R {
        MODER9_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - MODER10"]
    #[inline(always)]
    pub fn moder10(&self) -> MODER10_R {
        MODER10_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - MODER11"]
    #[inline(always)]
    pub fn moder11(&self) -> MODER11_R {
        MODER11_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - MODER12"]
    #[inline(always)]
    pub fn moder12(&self) -> MODER12_R {
        MODER12_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - MODER13"]
    #[inline(always)]
    pub fn moder13(&self) -> MODER13_R {
        MODER13_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - MODER14"]
    #[inline(always)]
    pub fn moder14(&self) -> MODER14_R {
        MODER14_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - MODER15"]
    #[inline(always)]
    pub fn moder15(&self) -> MODER15_R {
        MODER15_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - MODER0"]
    #[inline(always)]
    #[must_use]
    pub fn moder0(&mut self) -> MODER0_W<0> {
        MODER0_W::new(self)
    }
    #[doc = "Bits 2:3 - MODER1"]
    #[inline(always)]
    #[must_use]
    pub fn moder1(&mut self) -> MODER1_W<2> {
        MODER1_W::new(self)
    }
    #[doc = "Bits 4:5 - MODER2"]
    #[inline(always)]
    #[must_use]
    pub fn moder2(&mut self) -> MODER2_W<4> {
        MODER2_W::new(self)
    }
    #[doc = "Bits 6:7 - MODER3"]
    #[inline(always)]
    #[must_use]
    pub fn moder3(&mut self) -> MODER3_W<6> {
        MODER3_W::new(self)
    }
    #[doc = "Bits 8:9 - MODER4"]
    #[inline(always)]
    #[must_use]
    pub fn moder4(&mut self) -> MODER4_W<8> {
        MODER4_W::new(self)
    }
    #[doc = "Bits 10:11 - MODER5"]
    #[inline(always)]
    #[must_use]
    pub fn moder5(&mut self) -> MODER5_W<10> {
        MODER5_W::new(self)
    }
    #[doc = "Bits 12:13 - MODER6"]
    #[inline(always)]
    #[must_use]
    pub fn moder6(&mut self) -> MODER6_W<12> {
        MODER6_W::new(self)
    }
    #[doc = "Bits 14:15 - MODER7"]
    #[inline(always)]
    #[must_use]
    pub fn moder7(&mut self) -> MODER7_W<14> {
        MODER7_W::new(self)
    }
    #[doc = "Bits 16:17 - MODER8"]
    #[inline(always)]
    #[must_use]
    pub fn moder8(&mut self) -> MODER8_W<16> {
        MODER8_W::new(self)
    }
    #[doc = "Bits 18:19 - MODER9"]
    #[inline(always)]
    #[must_use]
    pub fn moder9(&mut self) -> MODER9_W<18> {
        MODER9_W::new(self)
    }
    #[doc = "Bits 20:21 - MODER10"]
    #[inline(always)]
    #[must_use]
    pub fn moder10(&mut self) -> MODER10_W<20> {
        MODER10_W::new(self)
    }
    #[doc = "Bits 22:23 - MODER11"]
    #[inline(always)]
    #[must_use]
    pub fn moder11(&mut self) -> MODER11_W<22> {
        MODER11_W::new(self)
    }
    #[doc = "Bits 24:25 - MODER12"]
    #[inline(always)]
    #[must_use]
    pub fn moder12(&mut self) -> MODER12_W<24> {
        MODER12_W::new(self)
    }
    #[doc = "Bits 26:27 - MODER13"]
    #[inline(always)]
    #[must_use]
    pub fn moder13(&mut self) -> MODER13_W<26> {
        MODER13_W::new(self)
    }
    #[doc = "Bits 28:29 - MODER14"]
    #[inline(always)]
    #[must_use]
    pub fn moder14(&mut self) -> MODER14_W<28> {
        MODER14_W::new(self)
    }
    #[doc = "Bits 30:31 - MODER15"]
    #[inline(always)]
    #[must_use]
    pub fn moder15(&mut self) -> MODER15_W<30> {
        MODER15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO port mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiob_moder](index.html) module"]
pub struct GPIOB_MODER_SPEC;
impl crate::RegisterSpec for GPIOB_MODER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpiob_moder::R](R) reader structure"]
impl crate::Readable for GPIOB_MODER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpiob_moder::W](W) writer structure"]
impl crate::Writable for GPIOB_MODER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPIOB_MODER to value 0xffff_ffff"]
impl crate::Resettable for GPIOB_MODER_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
