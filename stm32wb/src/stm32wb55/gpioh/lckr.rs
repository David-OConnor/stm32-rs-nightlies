#[doc = "Register `LCKR` reader"]
pub struct R(crate::R<LCKR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LCKR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LCKR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LCKR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LCKR` writer"]
pub struct W(crate::W<LCKR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LCKR_SPEC>;
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
impl From<crate::W<LCKR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LCKR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LCK0` reader - Port x lock bit y (y= 0..15)"]
pub type LCK0_R = crate::BitReader<bool>;
#[doc = "Field `LCK0` writer - Port x lock bit y (y= 0..15)"]
pub type LCK0_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCKR_SPEC, bool, O>;
#[doc = "Field `LCK1` reader - Port x lock bit y (y= 0..15)"]
pub type LCK1_R = crate::BitReader<bool>;
#[doc = "Field `LCK1` writer - Port x lock bit y (y= 0..15)"]
pub type LCK1_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCKR_SPEC, bool, O>;
#[doc = "Field `LCK3` reader - Port x lock bit y (y= 0..15)"]
pub type LCK3_R = crate::BitReader<bool>;
#[doc = "Field `LCK3` writer - Port x lock bit y (y= 0..15)"]
pub type LCK3_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCKR_SPEC, bool, O>;
#[doc = "Field `LCKK` reader - Port x lock bit y (y= 0..15)"]
pub type LCKK_R = crate::BitReader<bool>;
#[doc = "Field `LCKK` writer - Port x lock bit y (y= 0..15)"]
pub type LCKK_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCKR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Port x lock bit y (y= 0..15)"]
    #[inline(always)]
    pub fn lck0(&self) -> LCK0_R {
        LCK0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port x lock bit y (y= 0..15)"]
    #[inline(always)]
    pub fn lck1(&self) -> LCK1_R {
        LCK1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Port x lock bit y (y= 0..15)"]
    #[inline(always)]
    pub fn lck3(&self) -> LCK3_R {
        LCK3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 16 - Port x lock bit y (y= 0..15)"]
    #[inline(always)]
    pub fn lckk(&self) -> LCKK_R {
        LCKK_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port x lock bit y (y= 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn lck0(&mut self) -> LCK0_W<0> {
        LCK0_W::new(self)
    }
    #[doc = "Bit 1 - Port x lock bit y (y= 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn lck1(&mut self) -> LCK1_W<1> {
        LCK1_W::new(self)
    }
    #[doc = "Bit 3 - Port x lock bit y (y= 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn lck3(&mut self) -> LCK3_W<3> {
        LCK3_W::new(self)
    }
    #[doc = "Bit 16 - Port x lock bit y (y= 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn lckk(&mut self) -> LCKK_W<16> {
        LCKK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO port configuration lock register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lckr](index.html) module"]
pub struct LCKR_SPEC;
impl crate::RegisterSpec for LCKR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lckr::R](R) reader structure"]
impl crate::Readable for LCKR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lckr::W](W) writer structure"]
impl crate::Writable for LCKR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LCKR to value 0"]
impl crate::Resettable for LCKR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
