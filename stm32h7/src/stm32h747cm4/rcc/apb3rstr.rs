#[doc = "Register `APB3RSTR` reader"]
pub struct R(crate::R<APB3RSTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB3RSTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB3RSTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB3RSTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB3RSTR` writer"]
pub struct W(crate::W<APB3RSTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB3RSTR_SPEC>;
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
impl From<crate::W<APB3RSTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB3RSTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LTDCRST` reader - LTDC block reset"]
pub type LTDCRST_R = crate::BitReader<LTDCRST_A>;
#[doc = "LTDC block reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LTDCRST_A {
    #[doc = "1: Reset the selected module"]
    Reset = 1,
}
impl From<LTDCRST_A> for bool {
    #[inline(always)]
    fn from(variant: LTDCRST_A) -> Self {
        variant as u8 != 0
    }
}
impl LTDCRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LTDCRST_A> {
        match self.bits {
            true => Some(LTDCRST_A::Reset),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Reset`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == LTDCRST_A::Reset
    }
}
#[doc = "Field `LTDCRST` writer - LTDC block reset"]
pub type LTDCRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB3RSTR_SPEC, LTDCRST_A, O>;
impl<'a, const O: u8> LTDCRST_W<'a, O> {
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(LTDCRST_A::Reset)
    }
}
#[doc = "Field `DSIRST` reader - DSI block reset"]
pub use LTDCRST_R as DSIRST_R;
#[doc = "Field `DSIRST` writer - DSI block reset"]
pub use LTDCRST_W as DSIRST_W;
impl R {
    #[doc = "Bit 3 - LTDC block reset"]
    #[inline(always)]
    pub fn ltdcrst(&self) -> LTDCRST_R {
        LTDCRST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DSI block reset"]
    #[inline(always)]
    pub fn dsirst(&self) -> DSIRST_R {
        DSIRST_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - LTDC block reset"]
    #[inline(always)]
    #[must_use]
    pub fn ltdcrst(&mut self) -> LTDCRST_W<3> {
        LTDCRST_W::new(self)
    }
    #[doc = "Bit 4 - DSI block reset"]
    #[inline(always)]
    #[must_use]
    pub fn dsirst(&mut self) -> DSIRST_W<4> {
        DSIRST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RCC APB3 Peripheral Reset Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb3rstr](index.html) module"]
pub struct APB3RSTR_SPEC;
impl crate::RegisterSpec for APB3RSTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb3rstr::R](R) reader structure"]
impl crate::Readable for APB3RSTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb3rstr::W](W) writer structure"]
impl crate::Writable for APB3RSTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APB3RSTR to value 0"]
impl crate::Resettable for APB3RSTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
