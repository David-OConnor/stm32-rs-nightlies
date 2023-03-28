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
#[doc = "Field `LCK3` reader - Port x lock bit y (y= 0..15)"]
pub type LCK3_R = crate::BitReader<LCK3_A>;
#[doc = "Port x lock bit y (y= 0..15)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCK3_A {
    #[doc = "0: Port configuration not locked"]
    Unlocked = 0,
    #[doc = "1: Port configuration locked"]
    Locked = 1,
}
impl From<LCK3_A> for bool {
    #[inline(always)]
    fn from(variant: LCK3_A) -> Self {
        variant as u8 != 0
    }
}
impl LCK3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCK3_A {
        match self.bits {
            false => LCK3_A::Unlocked,
            true => LCK3_A::Locked,
        }
    }
    #[doc = "Checks if the value of the field is `Unlocked`"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == LCK3_A::Unlocked
    }
    #[doc = "Checks if the value of the field is `Locked`"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == LCK3_A::Locked
    }
}
#[doc = "Field `LCK3` writer - Port x lock bit y (y= 0..15)"]
pub type LCK3_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCKR_SPEC, LCK3_A, O>;
impl<'a, const O: u8> LCK3_W<'a, O> {
    #[doc = "Port configuration not locked"]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut W {
        self.variant(LCK3_A::Unlocked)
    }
    #[doc = "Port configuration locked"]
    #[inline(always)]
    pub fn locked(self) -> &'a mut W {
        self.variant(LCK3_A::Locked)
    }
}
#[doc = "Field `LCKK` reader - Port x lock bit y (y= 0..15)"]
pub type LCKK_R = crate::BitReader<LCKK_A>;
#[doc = "Port x lock bit y (y= 0..15)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCKK_A {
    #[doc = "0: Port configuration lock key not active"]
    NotActive = 0,
    #[doc = "1: Port configuration lock key active"]
    Active = 1,
}
impl From<LCKK_A> for bool {
    #[inline(always)]
    fn from(variant: LCKK_A) -> Self {
        variant as u8 != 0
    }
}
impl LCKK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCKK_A {
        match self.bits {
            false => LCKK_A::NotActive,
            true => LCKK_A::Active,
        }
    }
    #[doc = "Checks if the value of the field is `NotActive`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == LCKK_A::NotActive
    }
    #[doc = "Checks if the value of the field is `Active`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == LCKK_A::Active
    }
}
#[doc = "Field `LCKK` writer - Port x lock bit y (y= 0..15)"]
pub type LCKK_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCKR_SPEC, LCKK_A, O>;
impl<'a, const O: u8> LCKK_W<'a, O> {
    #[doc = "Port configuration lock key not active"]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(LCKK_A::NotActive)
    }
    #[doc = "Port configuration lock key active"]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(LCKK_A::Active)
    }
}
impl R {
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
