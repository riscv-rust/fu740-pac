#[doc = "Register `l2perfevent0` reader"]
pub struct R(crate::R<L2PERFEVENT0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<L2PERFEVENT0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<L2PERFEVENT0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<L2PERFEVENT0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `l2perfevent0` writer"]
pub struct W(crate::W<L2PERFEVENT0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<L2PERFEVENT0_SPEC>;
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
impl From<crate::W<L2PERFEVENT0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<L2PERFEVENT0_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The L2 performance event0 control register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [l2perfevent0](index.html) module"]
pub struct L2PERFEVENT0_SPEC;
impl crate::RegisterSpec for L2PERFEVENT0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [l2perfevent0::R](R) reader structure"]
impl crate::Readable for L2PERFEVENT0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [l2perfevent0::W](W) writer structure"]
impl crate::Writable for L2PERFEVENT0_SPEC {
    type Writer = W;
}
