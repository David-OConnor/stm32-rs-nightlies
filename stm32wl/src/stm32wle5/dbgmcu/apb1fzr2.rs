#[doc = "Register `APB1FZR2` reader"]
pub struct R(crate::R<APB1FZR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB1FZR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB1FZR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB1FZR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB1FZR2` writer"]
pub struct W(crate::W<APB1FZR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB1FZR2_SPEC>;
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
impl From<crate::W<APB1FZR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB1FZR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DBG_LPTIM2_STOP` reader - DBG_LPTIM2_STOP"]
pub type DBG_LPTIM2_STOP_R = crate::BitReader<DBG_LPTIM2_STOP_A>;
#[doc = "DBG_LPTIM2_STOP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBG_LPTIM2_STOP_A {
    #[doc = "0: LPTIMx counter clock is fed even if the core is halted"]
    Continue = 0,
    #[doc = "1: LPTIMx counter clock is stopped when the core is halted"]
    Stop = 1,
}
impl From<DBG_LPTIM2_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: DBG_LPTIM2_STOP_A) -> Self {
        variant as u8 != 0
    }
}
impl DBG_LPTIM2_STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBG_LPTIM2_STOP_A {
        match self.bits {
            false => DBG_LPTIM2_STOP_A::Continue,
            true => DBG_LPTIM2_STOP_A::Stop,
        }
    }
    #[doc = "Checks if the value of the field is `Continue`"]
    #[inline(always)]
    pub fn is_continue(&self) -> bool {
        *self == DBG_LPTIM2_STOP_A::Continue
    }
    #[doc = "Checks if the value of the field is `Stop`"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == DBG_LPTIM2_STOP_A::Stop
    }
}
#[doc = "Field `DBG_LPTIM2_STOP` writer - DBG_LPTIM2_STOP"]
pub type DBG_LPTIM2_STOP_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, APB1FZR2_SPEC, DBG_LPTIM2_STOP_A, O>;
impl<'a, const O: u8> DBG_LPTIM2_STOP_W<'a, O> {
    #[doc = "LPTIMx counter clock is fed even if the core is halted"]
    #[inline(always)]
    pub fn continue_(self) -> &'a mut W {
        self.variant(DBG_LPTIM2_STOP_A::Continue)
    }
    #[doc = "LPTIMx counter clock is stopped when the core is halted"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut W {
        self.variant(DBG_LPTIM2_STOP_A::Stop)
    }
}
#[doc = "Field `DBG_LPTIM3_STOP` reader - DBG_LPTIM3_STOP"]
pub use DBG_LPTIM2_STOP_R as DBG_LPTIM3_STOP_R;
#[doc = "Field `DBG_LPTIM3_STOP` writer - DBG_LPTIM3_STOP"]
pub use DBG_LPTIM2_STOP_W as DBG_LPTIM3_STOP_W;
impl R {
    #[doc = "Bit 5 - DBG_LPTIM2_STOP"]
    #[inline(always)]
    pub fn dbg_lptim2_stop(&self) -> DBG_LPTIM2_STOP_R {
        DBG_LPTIM2_STOP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - DBG_LPTIM3_STOP"]
    #[inline(always)]
    pub fn dbg_lptim3_stop(&self) -> DBG_LPTIM3_STOP_R {
        DBG_LPTIM3_STOP_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - DBG_LPTIM2_STOP"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_lptim2_stop(&mut self) -> DBG_LPTIM2_STOP_W<5> {
        DBG_LPTIM2_STOP_W::new(self)
    }
    #[doc = "Bit 6 - DBG_LPTIM3_STOP"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_lptim3_stop(&mut self) -> DBG_LPTIM3_STOP_W<6> {
        DBG_LPTIM3_STOP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DBGMCU CPU1 APB1 Peripheral Freeze Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb1fzr2](index.html) module"]
pub struct APB1FZR2_SPEC;
impl crate::RegisterSpec for APB1FZR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb1fzr2::R](R) reader structure"]
impl crate::Readable for APB1FZR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb1fzr2::W](W) writer structure"]
impl crate::Writable for APB1FZR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APB1FZR2 to value 0"]
impl crate::Resettable for APB1FZR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
